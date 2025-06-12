//! Thread-safe, non-blocking, "first one wins" flavor of `OnceCell`.
//!
//! If two threads race to initialize a type from the `race` module, they
//! don't block, execute initialization function together, but only one of
//! them stores the result.
//!
//! This module does not require `std` feature.
//!
//! # Atomic orderings
//!
//! All types in this module use `Acquire` and `Release`
//! [atomic orderings](Ordering) for all their operations. While this is not
//! strictly necessary for types other than `OnceBox`, it is useful for users as
//! it allows them to be certain that after `get` or `get_or_init` returns on
//! one thread, any side-effects caused by the setter thread prior to them
//! calling `set` or `get_or_init` will be made visible to that thread; without
//! it, it's possible for it to appear as if they haven't happened yet from the
//! getter thread's perspective. This is an acceptable tradeoff to make since
//! `Acquire` and `Release` have very little performance overhead on most
//! architectures versus `Relaxed`.

// The "atomic orderings" section of the documentation above promises
// "happens-before" semantics. This drives the choice of orderings in the uses
// of `compare_exchange` below. On success, the value was zero/null, so there
// was nothing to acquire (there is never any `Ordering::Release` store of 0).
// On failure, the value was nonzero, so it was initialized previously (perhaps
// on another thread) using `Ordering::Release`, so we must use
// `Ordering::Acquire` to ensure that store "happens-before" this load.

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;

use atomic::{AtomicPtr, AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::ptr;

/// A thread-safe cell which can be written to only once.
#[derive(Default, Debug)]
pub struct OnceNonZeroUsize {
    inner: AtomicUsize,
}

impl OnceNonZeroUsize {
    /// Creates a new empty cell.
    #[inline]
    pub const fn new() -> Self {
        Self { inner: AtomicUsize::new(0) }
    }

    /// Gets the underlying value.
    #[inline]
    pub fn get(&self) -> Option<NonZeroUsize> {
        let val = self.inner.load(Ordering::Acquire);
        NonZeroUsize::new(val)
    }

    /// Get the reference to the underlying value, without checking if the cell
    /// is initialized.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the cell is in initialized state, and that
    /// the contents are acquired by (synchronized to) this thread.
    pub unsafe fn get_unchecked(&self) -> NonZeroUsize {
        #[inline(always)]
        fn as_const_ptr(r: &AtomicUsize) -> *const usize {
            use core::mem::align_of;

            let p: *const AtomicUsize = r;
            // SAFETY: "This type has the same size and bit validity as
            // the underlying integer type, usize. However, the alignment of
            // this type is always equal to its size, even on targets where
            // usize has a lesser alignment."
            const _ALIGNMENT_COMPATIBLE: () =
                assert!(align_of::<AtomicUsize>() % align_of::<usize>() == 0);
            p.cast::<usize>()
        }

        // TODO(MSRV-1.70): Use `AtomicUsize::as_ptr().cast_const()`
        // See https://github.com/rust-lang/rust/issues/138246.
        let p = as_const_ptr(&self.inner);

        // SAFETY: The caller is responsible for ensuring that the value
        // was initialized and that the contents have been acquired by
        // this thread. Assuming that, we can assume there will be no
        // conflicting writes to the value since the value will never
        // change once initialized. This relies on the statement in
        // https://doc.rust-lang.org/1.83.0/core/sync/atomic/ that "(A
        // `compare_exchange` or `compare_exchange_weak` that does not
        // succeed is not considered a write."
        let val = unsafe { p.read() };

        // SAFETY: The caller is responsible for ensuring the value is
        // initialized and thus not zero.
        unsafe { NonZeroUsize::new_unchecked(val) }
    }

    /// Sets the contents of this cell to `value`.
    ///
    /// Returns `Ok(())` if the cell was empty and `Err(())` if it was
    /// full.
    #[inline]
    pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
        match self.compare_exchange(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if the cell was
    /// empty.
    ///
    /// If several threads concurrently run `get_or_init`, more than one `f` can
    /// be called. However, all threads will return the same value, produced by
    /// some `f`.
    pub fn get_or_init<F>(&self, f: F) -> NonZeroUsize
    where
        F: FnOnce() -> NonZeroUsize,
    {
        enum Void {}
        match self.get_or_try_init(|| Ok::<NonZeroUsize, Void>(f())) {
            Ok(val) => val,
            Err(void) => match void {},
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if
    /// the cell was empty. If the cell was empty and `f` failed, an
    /// error is returned.
    ///
    /// If several threads concurrently run `get_or_init`, more than one `f` can
    /// be called. However, all threads will return the same value, produced by
    /// some `f`.
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>
    where
        F: FnOnce() -> Result<NonZeroUsize, E>,
    {
        match self.get() {
            Some(it) => Ok(it),
            None => self.init(f),
        }
    }

    #[cold]
    #[inline(never)]
    fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
        let nz = f()?;
        let mut val = nz.get();
        if let Err(old) = self.compare_exchange(nz) {
            val = old;
        }
        Ok(unsafe { NonZeroUsize::new_unchecked(val) })
    }

    #[inline(always)]
    fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {
        self.inner.compare_exchange(0, val.get(), Ordering::Release, Ordering::Acquire)
    }
}

/// A thread-safe cell which can be written to only once.
#[derive(Default, Debug)]
pub struct OnceBool {
    inner: OnceNonZeroUsize,
}

impl OnceBool {
    /// Creates a new empty cell.
    #[inline]
    pub const fn new() -> Self {
        Self { inner: OnceNonZeroUsize::new() }
    }

    /// Gets the underlying value.
    #[inline]
    pub fn get(&self) -> Option<bool> {
        self.inner.get().map(Self::from_usize)
    }

    /// Sets the contents of this cell to `value`.
    ///
    /// Returns `Ok(())` if the cell was empty and `Err(())` if it was
    /// full.
    #[inline]
    pub fn set(&self, value: bool) -> Result<(), ()> {
        self.inner.set(Self::to_usize(value))
    }

    /// Gets the contents of the cell, initializing it with `f` if the cell was
    /// empty.
    ///
    /// If several threads concurrently run `get_or_init`, more than one `f` can
    /// be called. However, all threads will return the same value, produced by
    /// some `f`.
    pub fn get_or_init<F>(&self, f: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        Self::from_usize(self.inner.get_or_init(|| Self::to_usize(f())))
    }

    /// Gets the contents of the cell, initializing it with `f` if
    /// the cell was empty. If the cell was empty and `f` failed, an
    /// error is returned.
    ///
    /// If several threads concurrently run `get_or_init`, more than one `f` can
    /// be called. However, all threads will return the same value, produced by
    /// some `f`.
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>
    where
        F: FnOnce() -> Result<bool, E>,
    {
        self.inner.get_or_try_init(|| f().map(Self::to_usize)).map(Self::from_usize)
    }

    #[inline]
    fn from_usize(value: NonZeroUsize) -> bool {
        value.get() == 1
    }

    #[inline]
    fn to_usize(value: bool) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(if value { 1 } else { 2 }) }
    }
}

/// A thread-safe cell which can be written to only once.
pub struct OnceRef<'a, T> {
    inner: AtomicPtr<T>,
    ghost: PhantomData<UnsafeCell<&'a T>>,
}

// TODO: Replace UnsafeCell with SyncUnsafeCell once stabilized
unsafe impl<'a, T: Sync> Sync for OnceRef<'a, T> {}

impl<'a, T> core::fmt::Debug for OnceRef<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "OnceRef({:?})", self.inner)
    }
}

impl<'a, T> Default for OnceRef<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> OnceRef<'a, T> {
    /// Creates a new empty cell.
    pub const fn new() -> Self {
        Self { inner: AtomicPtr::new(ptr::null_mut()), ghost: PhantomData }
    }

    /// Gets a reference to the underlying value.
    pub fn get(&self) -> Option<&'a T> {
        let ptr = self.inner.load(Ordering::Acquire);
        unsafe { ptr.as_ref() }
    }

    /// Sets the contents of this cell to `value`.
    ///
    /// Returns `Ok(())` if the cell was empty and `Err(value)` if it was
    /// full.
    pub fn set(&self, value: &'a T) -> Result<(), ()> {
        match self.compare_exchange(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if the cell was
    /// empty.
    ///
    /// If several threads concurrently run `get_or_init`, more than one `f` can
    /// be called. However, all threads will return the same value, produced by
    /// some `f`.
    pub fn get_or_init<F>(&self, f: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
    {
        enum Void {}
        match self.get_or_try_init(|| Ok::<&'a T, Void>(f())) {
            Ok(val) => val,
            Err(void) => match void {},
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if
    /// the cell was empty. If the cell was empty and `f` failed, an
    /// error is returned.
    ///
    /// If several threads concurrently run `get_or_init`, more than one `f` can
    /// be called. However, all threads will return the same value, produced by
    /// some `f`.
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&'a T, E>
    where
        F: FnOnce() -> Result<&'a T, E>,
    {
        match self.get() {
            Some(val) => Ok(val),
            None => self.init(f),
        }
    }

    #[cold]
    #[inline(never)]
    fn init<E>(&self, f: impl FnOnce() -> Result<&'a T, E>) -> Result<&'a T, E> {
        let mut value: &'a T = f()?;
        if let Err(old) = self.compare_exchange(value) {
            value = unsafe { &*old };
        }
        Ok(value)
    }

    #[inline(always)]
    fn compare_exchange(&self, value: &'a T) -> Result<(), *const T> {
        self.inner
            .compare_exchange(
                ptr::null_mut(),
                <*const T>::cast_mut(value),
                Ordering::Release,
                Ordering::Acquire,
            )
            .map(|_: *mut T| ())
            .map_err(<*mut T>::cast_const)
    }

    /// ```compile_fail
    /// use once_cell::race::OnceRef;
    ///
    /// let mut l = OnceRef::new();
    ///
    /// {
    ///     let y = 2;
    ///     let mut r = OnceRef::new();
    ///     r.set(&y).unwrap();
    ///     core::mem::swap(&mut l, &mut r);
    /// }
    ///
    /// // l now contains a dangling reference to y
    /// eprintln!("uaf: {}", l.get().unwrap());
    /// ```
    fn _dummy() {}
}

#[cfg(feature = "alloc")]
pub use self::once_box::OnceBox;

#[cfg(feature = "alloc")]
pub mod once_box {
    use super::atomic::{AtomicPtr, Ordering};
    use core::{marker::PhantomData, ptr};

    use alloc::boxed::Box;

    /// A thread-safe cell which can be written to only once.
    pub struct OnceBox<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<Option<Box<T>>>,
    }

    impl<T> core::fmt::Debug for OnceBox<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "OnceBox({:?})", self.inner.load(Ordering::Relaxed))
        }
    }

    impl<T> Default for OnceBox<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Drop for OnceBox<T> {
        fn drop(&mut self) {
            let ptr = *self.inner.get_mut();
            if !ptr.is_null() {
                drop(unsafe { Box::from_raw(ptr) })
            }
        }
    }

    impl<T> OnceBox<T> {
        /// Creates a new empty cell.
        pub const fn new() -> Self {
            Self { inner: AtomicPtr::new(ptr::null_mut()), ghost: PhantomData }
        }

        /// Creates a new cell with the given value.
        pub fn with_value(value: Box<T>) -> Self {
            Self { inner: AtomicPtr::new(Box::into_raw(value)), ghost: PhantomData }
        }

        /// Gets a reference to the underlying value.
        pub fn get(&self) -> Option<&T> {
            let ptr = self.inner.load(Ordering::Acquire);
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }

        /// Sets the contents of this cell to `value`.
        ///
        /// Returns `Ok(())` if the cell was empty and `Err(value)` if it was
        /// full.
        pub fn set(&self, value: Box<T>) -> Result<(), Box<T>> {
            let ptr = Box::into_raw(value);
            let exchange = self.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::Release,
                Ordering::Acquire,
            );
            if exchange.is_err() {
                let value = unsafe { Box::from_raw(ptr) };
                return Err(value);
            }
            Ok(())
        }

        /// Gets the contents of the cell, initializing it with `f` if the cell was
        /// empty.
        ///
        /// If several threads concurrently run `get_or_init`, more than one `f` can
        /// be called. However, all threads will return the same value, produced by
        /// some `f`.
        pub fn get_or_init<F>(&self, f: F) -> &T
        where
            F: FnOnce() -> Box<T>,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<Box<T>, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }

        /// Gets the contents of the cell, initializing it with `f` if
        /// the cell was empty. If the cell was empty and `f` failed, an
        /// error is returned.
        ///
        /// If several threads concurrently run `get_or_init`, more than one `f` can
        /// be called. However, all threads will return the same value, produced by
        /// some `f`.
        pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<Box<T>, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f)
            }
        }

        #[cold]
        #[inline(never)]
        fn init<E>(&self, f: impl FnOnce() -> Result<Box<T>, E>) -> Result<&T, E> {
            let val = f()?;
            let mut ptr = Box::into_raw(val);
            let exchange = self.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::Release,
                Ordering::Acquire,
            );
            if let Err(old) = exchange {
                drop(unsafe { Box::from_raw(ptr) });
                ptr = old;
            }
            Ok(unsafe { &*ptr })
        }
    }

    unsafe impl<T: Sync + Send> Sync for OnceBox<T> {}

    impl<T: Clone> Clone for OnceBox<T> {
        fn clone(&self) -> Self {
            match self.get() {
                Some(value) => OnceBox::with_value(Box::new(value.clone())),
                None => OnceBox::new(),
            }
        }
    }

    /// ```compile_fail
    /// struct S(*mut ());
    /// unsafe impl Sync for S {}
    ///
    /// fn share<T: Sync>(_: &T) {}
    /// share(&once_cell::race::OnceBox::<S>::new());
    /// ```
    pub fn _dummy() {}
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::From;
	use std::ops::Drop;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_3() {
//     rusty_monitor::set_test_id(3);
//     let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncebox_0: crate::race::once_box::OnceBox<i64> = crate::race::once_box::OnceBox::default();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<i64> = &mut oncebox_0;
//     let mut isize_0: isize = 26303isize;
//     let mut lazy_0: crate::unsync::Lazy<std::result::Result<&isize, isize>, isize> = crate::unsync::Lazy::new(isize_0);
//     let mut lazy_0_ref_0: &crate::unsync::Lazy<std::result::Result<&isize, isize>, isize> = &mut lazy_0;
//     let mut oncecell_2: crate::unsync::OnceCell<std::result::Result<&isize, isize>> = crate::unsync::OnceCell::new();
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_1;
//     let mut isize_1: isize = -2805isize;
//     let mut isize_2: isize = 8321isize;
//     let mut isize_3: isize = 2178isize;
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_3);
//     let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
//     let mut isize_4: isize = -1473isize;
//     let mut box_1: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_2: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_1);
//     let mut oncebox_2_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_2;
//     let mut option_0: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_2_ref_0);
//     let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_4);
//     let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_3_ref_0, isize_2);
//     let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_1);
//     let mut result_1: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_1_ref_0, box_0);
//     let mut oncebox_3: crate::race::once_box::OnceBox<i64> = crate::race::once_box::OnceBox::clone(oncebox_0_ref_0);
//     let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_0);
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut oncecell_5_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_5;
//     let mut isize_5: &isize = crate::sync::OnceCell::wait(oncecell_5_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_6() {
//     rusty_monitor::set_test_id(6);
//     let mut isize_0: isize = -767isize;
//     let mut isize_1: isize = 5393isize;
//     let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_1);
//     let mut oncecell_0_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_0;
//     let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_1_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_1;
//     let mut isize_2: isize = -2410isize;
//     let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_2);
//     let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut isize_3: isize = 10956isize;
//     let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_3);
//     let mut oncecell_3_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_3;
//     let mut isize_4: isize = 4126isize;
//     let mut oncecell_4: crate::sync::OnceCell<dyn std::ops::FnMut> = crate::sync::OnceCell::new();
//     let mut oncecell_4_ref_0: &crate::sync::OnceCell<dyn std::ops::FnMut> = &mut oncecell_4;
//     let mut option_0: std::option::Option<&mut dyn std::ops::FnMut> = crate::sync::OnceCell::get(oncecell_4_ref_0);
//     let mut u8_0: u8 = 112u8;
//     let mut oncecell_5: crate::sync::OnceCell<u8> = crate::sync::OnceCell::from(u8_0);
//     let mut oncecell_5_ref_0: &crate::sync::OnceCell<u8> = &mut oncecell_5;
//     let mut oncecell_6: crate::sync::OnceCell<u8> = crate::sync::OnceCell::default();
//     let mut oncecell_6_ref_0: &mut crate::sync::OnceCell<u8> = &mut oncecell_6;
//     crate::sync::OnceCell::clone_from(oncecell_6_ref_0, oncecell_5_ref_0);
//     let mut oncecell_7: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     let mut oncecell_8: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_4);
//     let mut option_1: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_3_ref_0);
//     let mut option_2: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
//     crate::race::once_box::_dummy();
//     let mut option_3: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_2_ref_0);
//     let mut option_4: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_1_ref_0);
//     let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_0_ref_0, isize_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut oncecell_0: crate::sync::OnceCell<i16> = crate::sync::OnceCell::new();
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<i16> = &mut oncecell_0;
    let mut oncecell_1: crate::unsync::OnceCell<std::result::Result<(), ()>> = crate::unsync::OnceCell::new();
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<std::result::Result<(), ()>> = &mut oncecell_1;
    let mut i128_0: i128 = -9859i128;
    let mut oncecell_2: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::with_value(i128_0);
    let mut oncecell_2_ref_0: &crate::unsync::OnceCell<i128> = &mut oncecell_2;
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
    let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut isize_0: isize = -2179isize;
    let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_0);
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
    let mut oncebool_1_ref_0: &crate::race::OnceBool = &mut oncebool_1;
    let mut isize_1: isize = 3386isize;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    crate::race::once_box::_dummy();
    let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_3_ref_0, isize_1);
    let mut option_0: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_1_ref_0);
    let mut result_1: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
    let mut oncecell_4_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_4;
    let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_4_ref_0);
    let mut option_2: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_0_ref_0);
    let mut oncecell_5: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::clone(oncecell_2_ref_0);
    let mut bool_0: bool = std::option::Option::unwrap(option_0);
    let mut oncecell_6: crate::sync::OnceCell<i16> = crate::sync::OnceCell::clone(oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_11() {
//     rusty_monitor::set_test_id(11);
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_1_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_1;
//     let mut oncecell_2: crate::unsync::OnceCell<u8> = crate::unsync::OnceCell::default();
//     let mut oncecell_2_ref_0: &crate::unsync::OnceCell<u8> = &mut oncecell_2;
//     let mut u8_0: u8 = 32u8;
//     let mut oncecell_3: crate::unsync::OnceCell<u8> = crate::unsync::OnceCell::with_value(u8_0);
//     let mut oncecell_3_ref_0: &mut crate::unsync::OnceCell<u8> = &mut oncecell_3;
//     let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_4_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_4;
//     let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_5_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_5;
//     let mut isize_0: isize = -13333isize;
//     let mut isize_1: isize = 5446isize;
//     let mut isize_2: isize = -9739isize;
//     let mut oncecell_6: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_6_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_6;
//     let mut isize_3: isize = -8737isize;
//     let mut oncecell_7: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_3);
//     crate::race::once_box::_dummy();
//     let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_6_ref_0, isize_2);
//     crate::race::OnceRef::_dummy();
//     let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_1);
//     let mut oncecell_7_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_7;
//     let mut result_1: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_7_ref_0, isize_0);
//     crate::sync::OnceCell::clone_from(oncecell_5_ref_0, oncecell_4_ref_0);
//     crate::race::once_box::_dummy();
//     crate::unsync::OnceCell::clone_from(oncecell_3_ref_0, oncecell_2_ref_0);
//     let mut option_0: std::option::Option<&mut isize> = crate::sync::OnceCell::get_mut(oncecell_1_ref_0);
//     let mut isize_4: &isize = crate::sync::OnceCell::wait(oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_13() {
//     rusty_monitor::set_test_id(13);
//     let mut isize_0: isize = 6598isize;
//     let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_0);
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut oncenonzerousize_0_ref_0: &crate::race::OnceNonZeroUsize = &mut oncenonzerousize_0;
//     let mut f32_0: f32 = -4019.322204f32;
//     let mut oncecell_0: crate::sync::OnceCell<f32> = crate::sync::OnceCell::from(f32_0);
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<f32> = &mut oncecell_0;
//     let mut oncecell_1: crate::sync::OnceCell<f32> = crate::sync::OnceCell::new();
//     let mut oncecell_1_ref_0: &mut crate::sync::OnceCell<f32> = &mut oncecell_1;
//     let mut oncenonzerousize_1: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_1};
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut isize_1: isize = 29704isize;
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_2_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_2;
//     let mut isize_2: isize = -3710isize;
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
//     let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_3_ref_0, isize_2);
//     let mut lazy_1: crate::sync::Lazy<&str> = crate::sync::Lazy::default();
//     crate::race::once_box::_dummy();
//     let mut oncebox_0_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     crate::race::once_box::OnceBox::drop(oncebox_0_ref_0);
//     let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut result_1: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_2_ref_0, isize_1);
//     let mut option_0: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_0_ref_0);
//     crate::sync::OnceCell::clone_from(oncecell_1_ref_0, oncecell_0_ref_0);
//     let mut result_2: std::result::Result<isize, isize> = crate::sync::Lazy::into_value(lazy_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_15() {
//     rusty_monitor::set_test_id(15);
//     let mut isize_0: isize = -7785isize;
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut u128_0: u128 = 9458u128;
//     let mut oncecell_1: crate::unsync::OnceCell<u128> = crate::unsync::OnceCell::from(u128_0);
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<u128> = &mut oncecell_1;
//     let mut u128_1: u128 = 7341u128;
//     let mut oncecell_2: crate::unsync::OnceCell<u128> = crate::unsync::OnceCell::from(u128_1);
//     let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<u128> = &mut oncecell_2;
//     let mut oncecell_3: crate::unsync::OnceCell<std::result::Result<(), isize>> = crate::unsync::OnceCell::default();
//     let mut oncecell_3_ref_0: &crate::unsync::OnceCell<std::result::Result<(), isize>> = &mut oncecell_3;
//     let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_4_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_4;
//     let mut isize_1: isize = -10146isize;
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
//     let mut oncebox_0_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut bool_0: bool = true;
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut result_0: std::result::Result<(), ()> = crate::race::OnceBool::set(oncebool_0_ref_0, bool_0);
//     crate::race::once_box::OnceBox::drop(oncebox_0_ref_0);
//     let mut tuple_0: () = std::result::Result::unwrap(result_0);
//     crate::race::once_box::_dummy();
//     let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_1);
//     let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut option_0: std::option::Option<isize> = crate::sync::OnceCell::take(oncecell_4_ref_0);
//     let mut oncebool_2: crate::race::OnceBool = crate::race::OnceBool::new();
//     crate::unsync::OnceCell::clone_from(oncecell_2_ref_0, oncecell_1_ref_0);
//     let mut lazy_1: crate::sync::Lazy<f64> = crate::sync::Lazy::default();
//     let mut result_1: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_0_ref_0, isize_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_19() {
//     rusty_monitor::set_test_id(19);
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut isize_0: isize = -6125isize;
//     let mut oncebox_1: crate::race::once_box::OnceBox<u128> = crate::race::once_box::OnceBox::default();
//     let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<u128> = &mut oncebox_1;
//     let mut isize_1: isize = 9888isize;
//     let mut u8_0: u8 = 73u8;
//     let mut oncecell_0: crate::sync::OnceCell<u8> = crate::sync::OnceCell::from(u8_0);
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<u8> = &mut oncecell_0;
//     let mut oncecell_1: crate::sync::OnceCell<u8> = crate::sync::OnceCell::default();
//     let mut oncecell_1_ref_0: &crate::sync::OnceCell<u8> = &mut oncecell_1;
//     let mut isize_2: isize = -2833isize;
//     let mut lazy_0: crate::sync::Lazy<std::result::Result<bool, isize>, isize> = crate::sync::Lazy::new(isize_2);
//     let mut lazy_0_ref_0: &crate::sync::Lazy<std::result::Result<bool, isize>, isize> = &mut lazy_0;
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut isize_3: isize = 4178isize;
//     let mut isize_4: isize = -9251isize;
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_4);
//     let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
//     crate::race::OnceRef::_dummy();
//     let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
//     let mut option_0: std::option::Option<&isize> = crate::race::OnceRef::get(onceref_0_ref_0);
//     let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_3_ref_0, isize_3);
//     let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_2);
//     let mut tuple_0: () = std::result::Result::unwrap(result_0);
//     let mut bool_0: bool = crate::sync::OnceCell::eq(oncecell_1_ref_0, oncecell_0_ref_0);
//     let mut lazy_1: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_1);
//     let mut oncebox_2: crate::race::once_box::OnceBox<u128> = crate::race::once_box::OnceBox::clone(oncebox_1_ref_0);
//     let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut isize_0: isize = 4527isize;
    let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut isize_1: isize = 567isize;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
    let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_2;
    let mut oncecell_3: crate::sync::OnceCell<f64> = crate::sync::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_3;
    let mut oncecell_4: crate::sync::OnceCell<f64> = crate::sync::OnceCell::new();
    let mut oncecell_4_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_4;
    let mut isize_2: isize = -8016isize;
    let mut isize_2_ref_0: &isize = &mut isize_2;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut isize_3: isize = -2931isize;
    let mut isize_3_ref_0: &isize = &mut isize_3;
    let mut onceref_1: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_1_ref_0: &crate::race::OnceRef<isize> = &mut onceref_1;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
    let mut onceref_2: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_2_ref_0: &crate::race::OnceRef<isize> = &mut onceref_2;
    let mut option_0: std::option::Option<&isize> = crate::race::OnceRef::get(onceref_2_ref_0);
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
    let mut bool_0: bool = crate::sync::OnceCell::eq(oncecell_4_ref_0, oncecell_3_ref_0);
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut option_1: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_2_ref_0);
    let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut result_0: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_1_ref_0, isize_1);
    let mut option_2: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_0);
    let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut isize_0: isize = 15275isize;
    let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_0);
    let mut isize_1: isize = -10224isize;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_1);
    let mut oncecell_0_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_0;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut bool_0: bool = false;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
    let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
    let mut isize_2: isize = 4231isize;
    let mut oncenonzerousize_1: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
    let mut oncenonzerousize_1_ref_0: &crate::race::OnceNonZeroUsize = &mut oncenonzerousize_1;
    let mut oncenonzerousize_2: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut isize_3: isize = 2224isize;
    let mut isize_3_ref_0: &isize = &mut isize_3;
    let mut onceref_1: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_1_ref_0: &crate::race::OnceRef<isize> = &mut onceref_1;
    let mut oncenonzerousize_3: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_3};
    let mut result_0: std::result::Result<(), ()> = crate::race::OnceRef::set(onceref_1_ref_0, isize_3_ref_0);
    let mut oncebool_2: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_2};
    let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_2);
    let mut result_1: std::result::Result<(), ()> = crate::race::OnceBool::set(oncebool_0_ref_0, bool_0);
    let mut option_0: std::option::Option<&isize> = crate::race::OnceRef::get(onceref_0_ref_0);
    let mut option_1: std::option::Option<isize> = crate::sync::OnceCell::take(oncecell_0_ref_0);
    let mut result_2: std::result::Result<isize, isize> = crate::sync::Lazy::into_value(lazy_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut oncenonzerousize_4: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_22() {
//     rusty_monitor::set_test_id(22);
//     let mut isize_0: isize = 2947isize;
//     let mut isize_1: isize = -8426isize;
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_1);
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_1;
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_2;
//     let mut u128_0: u128 = 4875u128;
//     let mut oncecell_3: crate::sync::OnceCell<u128> = crate::sync::OnceCell::with_value(u128_0);
//     let mut oncecell_3_ref_0: &crate::sync::OnceCell<u128> = &mut oncecell_3;
//     let mut isize_2: isize = 3593isize;
//     let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     let mut oncecell_4_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_4;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut box_1: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_1);
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_5_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_5;
//     let mut i16_0: i16 = 3326i16;
//     let mut oncecell_6: crate::unsync::OnceCell<i16> = crate::unsync::OnceCell::with_value(i16_0);
//     let mut oncecell_6_ref_0: &crate::unsync::OnceCell<i16> = &mut oncecell_6;
//     let mut oncecell_7: crate::unsync::OnceCell<i16> = crate::unsync::OnceCell::default();
//     let mut oncecell_7_ref_0: &crate::unsync::OnceCell<i16> = &mut oncecell_7;
//     let mut bool_0: bool = crate::unsync::OnceCell::eq(oncecell_7_ref_0, oncecell_6_ref_0);
//     let mut isize_3: &isize = crate::sync::OnceCell::wait(oncecell_5_ref_0);
//     let mut result_0: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_0_ref_0, box_0);
//     let mut result_1: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_4_ref_0, isize_2);
//     let mut oncecell_8: crate::sync::OnceCell<u128> = crate::sync::OnceCell::clone(oncecell_3_ref_0);
//     crate::unsync::OnceCell::clone_from(oncecell_2_ref_0, oncecell_1_ref_0);
//     let mut result_2: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_0_ref_0, isize_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut isize_0: isize = -5420isize;
    let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_0);
    let mut isize_1: isize = 851isize;
    let mut isize_2: isize = 3715isize;
    let mut lazy_1: crate::sync::Lazy<std::result::Result<&isize, isize>, isize> = crate::sync::Lazy::new(isize_2);
    let mut lazy_1_ref_0: &crate::sync::Lazy<std::result::Result<&isize, isize>, isize> = &mut lazy_1;
    let mut oncecell_0: crate::sync::OnceCell<u16> = crate::sync::OnceCell::new();
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<u16> = &mut oncecell_0;
    let mut oncecell_1: crate::sync::OnceCell<u16> = crate::sync::OnceCell::new();
    let mut oncecell_1_ref_0: &mut crate::sync::OnceCell<u16> = &mut oncecell_1;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut oncecell_2: crate::sync::OnceCell<f64> = crate::sync::OnceCell::new();
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_2;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut isize_3: isize = -3241isize;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_3_ref_0, isize_3);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut oncecell_4: crate::sync::OnceCell<f64> = crate::sync::OnceCell::clone(oncecell_2_ref_0);
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
    crate::sync::OnceCell::clone_from(oncecell_1_ref_0, oncecell_0_ref_0);
    let mut oncecell_4_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_4;
    let mut oncecell_5: crate::sync::OnceCell<f64> = crate::sync::OnceCell::clone(oncecell_4_ref_0);
    let mut oncecell_5_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_5;
    let mut option_0: std::option::Option<&f64> = crate::sync::OnceCell::get(oncecell_5_ref_0);
    let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
    let mut f64_0: &f64 = std::option::Option::unwrap(option_0);
    let mut oncecell_6: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_1);
    let mut result_1: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_26() {
//     rusty_monitor::set_test_id(26);
//     let mut i128_0: i128 = 5188i128;
//     let mut oncecell_0: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::from(i128_0);
//     let mut oncecell_0_ref_0: &crate::unsync::OnceCell<i128> = &mut oncecell_0;
//     let mut oncecell_1: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::default();
//     let mut oncecell_1_ref_0: &mut crate::unsync::OnceCell<i128> = &mut oncecell_1;
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut isize_0: isize = 133isize;
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut isize_1: isize = -4354isize;
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
//     let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_2;
//     let mut lazy_0: crate::sync::Lazy<bool> = crate::sync::Lazy::default();
//     let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_2_ref_0);
//     let mut option_1: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
//     let mut isize_2: &isize = std::option::Option::unwrap(option_1);
//     let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     crate::race::OnceRef::_dummy();
//     let mut oncecell_3_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_3;
//     let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::clone(oncecell_3_ref_0);
//     let mut lazy_1: crate::unsync::Lazy<u64> = crate::unsync::Lazy::default();
//     let mut oncecell_4_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_4;
//     let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::clone(oncecell_4_ref_0);
//     let mut oncecell_5_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_5;
//     let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_5_ref_0, isize_0);
//     let mut lazy_2: crate::unsync::Lazy<usize> = crate::unsync::Lazy::default();
//     let mut option_2: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_0_ref_0);
//     let mut bool_0: bool = std::option::Option::unwrap(option_2);
//     let mut isize_3: &isize = std::result::Result::unwrap(result_0);
//     crate::unsync::OnceCell::clone_from(oncecell_1_ref_0, oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_31() {
//     rusty_monitor::set_test_id(31);
//     let mut isize_0: isize = 9679isize;
//     let mut isize_1: isize = 5851isize;
//     let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
//     let mut oncecell_0_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_0;
//     let mut bool_0: bool = true;
//     let mut oncecell_1: crate::unsync::OnceCell<bool> = crate::unsync::OnceCell::with_value(bool_0);
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<bool> = &mut oncecell_1;
//     let mut oncecell_2: crate::unsync::OnceCell<bool> = crate::unsync::OnceCell::default();
//     let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<bool> = &mut oncecell_2;
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     let mut oncecell_4: crate::sync::OnceCell<bool> = crate::sync::OnceCell::default();
//     let mut oncecell_4_ref_0: &crate::sync::OnceCell<bool> = &mut oncecell_4;
//     let mut isize_2: isize = 4371isize;
//     let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_2);
//     let mut oncecell_5_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_5;
//     let mut isize_3: isize = 1510isize;
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
//     let mut oncebool_1_ref_0: &crate::race::OnceBool = &mut oncebool_1;
//     let mut oncecell_6: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_3);
//     let mut option_0: std::option::Option<isize> = crate::sync::OnceCell::take(oncecell_5_ref_0);
//     let mut oncecell_7: crate::sync::OnceCell<bool> = crate::sync::OnceCell::clone(oncecell_4_ref_0);
//     let mut oncecell_7_ref_0: &crate::sync::OnceCell<bool> = &mut oncecell_7;
//     let mut option_1: std::option::Option<&bool> = crate::sync::OnceCell::get(oncecell_7_ref_0);
//     let mut option_2: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_3);
//     let mut option_3: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_0_ref_0);
//     let mut oncecell_6_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_6;
//     let mut option_4: std::option::Option<isize> = crate::sync::OnceCell::take(oncecell_6_ref_0);
//     let mut oncecell_8: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     crate::unsync::OnceCell::clone_from(oncecell_2_ref_0, oncecell_1_ref_0);
//     let mut result_0: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_0_ref_0, isize_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_33() {
//     rusty_monitor::set_test_id(33);
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_0_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut isize_0: isize = 14688isize;
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut tuple_0: () = ();
//     let mut lazy_0: crate::sync::Lazy<(), ()> = crate::sync::Lazy::new(tuple_0);
//     let mut result_0: std::result::Result<(), ()> = crate::sync::Lazy::into_value(lazy_0);
//     let mut oncecell_1: crate::sync::OnceCell<std::result::Result<(), ()>> = crate::sync::OnceCell::with_value(result_0);
//     let mut oncecell_1_ref_0: &crate::sync::OnceCell<std::result::Result<(), ()>> = &mut oncecell_1;
//     let mut isize_1: isize = -459isize;
//     let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
//     let mut isize_2: isize = -5659isize;
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_2);
//     let mut isize_3: isize = 3879isize;
//     let mut oncecell_4: crate::unsync::OnceCell<std::result::Result<bool, isize>> = crate::unsync::OnceCell::default();
//     let mut oncecell_4_ref_0: &crate::unsync::OnceCell<std::result::Result<bool, isize>> = &mut oncecell_4;
//     let mut lazy_1: crate::sync::Lazy<u32> = crate::sync::Lazy::default();
//     let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut oncecell_6: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_3);
//     let mut option_0: std::option::Option<isize> = crate::sync::OnceCell::into_inner(oncecell_3);
//     let mut result_1: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_2_ref_0, isize_1);
//     let mut option_1: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_0_ref_0);
//     let mut oncecell_7: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_6_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_6;
//     let mut option_2: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_6_ref_0);
//     let mut oncebool_2: crate::race::OnceBool = crate::race::OnceBool::new();
//     let mut lazy_2: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_0);
//     let mut option_3: std::option::Option<isize> = crate::sync::OnceCell::take(oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_34() {
//     rusty_monitor::set_test_id(34);
//     let mut isize_0: isize = -7031isize;
//     let mut isize_1: isize = 11074isize;
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_1);
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut isize_2: isize = -3573isize;
//     let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_2);
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut oncebox_1: crate::race::once_box::OnceBox<i64> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<i64> = &mut oncebox_1;
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut isize_3: isize = -8495isize;
//     let mut str_0: &str = "jsc";
//     let mut oncecell_1: crate::unsync::OnceCell<&str> = crate::unsync::OnceCell::with_value(str_0);
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<&str> = &mut oncecell_1;
//     let mut str_1: &str = "8tEs";
//     let mut oncecell_2: crate::unsync::OnceCell<&str> = crate::unsync::OnceCell::from(str_1);
//     let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<&str> = &mut oncecell_2;
//     let mut isize_4: isize = 8031isize;
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_4);
//     let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
//     let mut isize_5: &isize = crate::sync::OnceCell::wait(oncecell_3_ref_0);
//     crate::unsync::OnceCell::clone_from(oncecell_2_ref_0, oncecell_1_ref_0);
//     let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_3);
//     let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool::new();
//     let mut oncenonzerousize_1: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut oncebox_2: crate::race::once_box::OnceBox<i64> = crate::race::once_box::OnceBox::clone(oncebox_1_ref_0);
//     let mut oncebox_2_ref_0: &crate::race::once_box::OnceBox<i64> = &mut oncebox_2;
//     let mut option_0: std::option::Option<&i64> = crate::race::once_box::OnceBox::get(oncebox_2_ref_0);
//     let mut result_0: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
//     let mut result_1: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_0_ref_0, isize_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_35() {
//     rusty_monitor::set_test_id(35);
//     let mut isize_0: isize = -7487isize;
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_0_ref_0, isize_0);
//     let mut oncecell_1: crate::unsync::OnceCell<std::result::Result<&isize, (&isize, isize)>> = crate::unsync::OnceCell::from(result_0);
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<std::result::Result<&isize, (&isize, isize)>> = &mut oncecell_1;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut i128_0: i128 = 3848i128;
//     let mut oncecell_2: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::from(i128_0);
//     let mut oncecell_2_ref_0: &crate::unsync::OnceCell<i128> = &mut oncecell_2;
//     let mut oncecell_3: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::default();
//     let mut oncecell_3_ref_0: &crate::unsync::OnceCell<i128> = &mut oncecell_3;
//     let mut oncecell_4: crate::unsync::OnceCell<std::result::Result<(), isize>> = crate::unsync::OnceCell::new();
//     let mut oncecell_4_ref_0: &crate::unsync::OnceCell<std::result::Result<(), isize>> = &mut oncecell_4;
//     let mut u16_0: u16 = 3338u16;
//     let mut oncecell_5: crate::sync::OnceCell<u16> = crate::sync::OnceCell::with_value(u16_0);
//     let mut oncecell_5_ref_0: &crate::sync::OnceCell<u16> = &mut oncecell_5;
//     let mut u16_1: u16 = 9239u16;
//     let mut oncecell_6: crate::sync::OnceCell<u16> = crate::sync::OnceCell::from(u16_1);
//     let mut oncecell_6_ref_0: &mut crate::sync::OnceCell<u16> = &mut oncecell_6;
//     let mut isize_1: isize = 26985isize;
//     let mut isize_1_ref_0: &isize = &mut isize_1;
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
//     let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
//     let mut result_1: std::result::Result<(), ()> = crate::race::OnceRef::set(onceref_0_ref_0, isize_1_ref_0);
//     crate::sync::OnceCell::clone_from(oncecell_6_ref_0, oncecell_5_ref_0);
//     let mut tuple_0: () = std::result::Result::unwrap(result_1);
//     let mut oncecell_7: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::clone(oncecell_3_ref_0);
//     let mut oncecell_7_ref_0: &mut crate::unsync::OnceCell<i128> = &mut oncecell_7;
//     crate::unsync::OnceCell::clone_from(oncecell_7_ref_0, oncecell_2_ref_0);
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
//     let mut oncebox_0_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     crate::race::once_box::OnceBox::drop(oncebox_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_38() {
//     rusty_monitor::set_test_id(38);
//     let mut isize_0: isize = 20776isize;
//     let mut isize_1: isize = 5774isize;
//     let mut isize_2: isize = 9977isize;
//     let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_2);
//     let mut oncecell_0_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_0;
//     let mut isize_3: isize = -1149isize;
//     let mut lazy_0: crate::sync::Lazy<std::result::Result<&isize, isize>, isize> = crate::sync::Lazy::new(isize_3);
//     let mut lazy_0_ref_0: &crate::sync::Lazy<std::result::Result<&isize, isize>, isize> = &mut lazy_0;
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool {inner: oncenonzerousize_0};
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut isize_4: isize = 5928isize;
//     let mut bool_0: bool = false;
//     let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool::new();
//     let mut oncebool_1_ref_0: &crate::race::OnceBool = &mut oncebool_1;
//     let mut result_0: std::result::Result<(), ()> = crate::race::OnceBool::set(oncebool_1_ref_0, bool_0);
//     let mut oncecell_1: crate::unsync::OnceCell<std::result::Result<(), ()>> = crate::unsync::OnceCell::with_value(result_0);
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<std::result::Result<(), ()>> = &mut oncecell_1;
//     let mut isize_5: isize = 2944isize;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_5);
//     let mut oncecell_2_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_2;
//     let mut result_1: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_2_ref_0, isize_4);
//     let mut option_0: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_0_ref_0);
//     let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncebool_2: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut result_2: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_0_ref_0, isize_1);
//     let mut isize_6: &isize = std::result::Result::unwrap(result_2);
//     let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_42() {
//     rusty_monitor::set_test_id(42);
//     let mut u16_0: u16 = 3110u16;
//     let mut oncecell_0: crate::sync::OnceCell<u16> = crate::sync::OnceCell::from(u16_0);
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<u16> = &mut oncecell_0;
//     let mut u16_1: u16 = 7920u16;
//     let mut oncecell_1: crate::sync::OnceCell<u16> = crate::sync::OnceCell::from(u16_1);
//     let mut oncecell_1_ref_0: &crate::sync::OnceCell<u16> = &mut oncecell_1;
//     let mut oncecell_2: crate::sync::OnceCell<bool> = crate::sync::OnceCell::new();
//     let mut oncecell_2_ref_0: &crate::sync::OnceCell<bool> = &mut oncecell_2;
//     let mut bool_0: bool = true;
//     let mut oncecell_3: crate::sync::OnceCell<bool> = crate::sync::OnceCell::with_value(bool_0);
//     let mut oncecell_3_ref_0: &mut crate::sync::OnceCell<bool> = &mut oncecell_3;
//     let mut isize_0: isize = -13867isize;
//     let mut isize_1: isize = 6646isize;
//     let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_1);
//     let mut oncecell_4_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_4;
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut isize_2: isize = 1548isize;
//     let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_2);
//     let mut isize_3: isize = 19735isize;
//     let mut oncecell_6: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_3);
//     let mut oncecell_7: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
//     crate::race::OnceRef::_dummy();
//     let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::clone(oncebox_0_ref_0);
//     let mut oncecell_8: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncenonzerousize_1: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
//     let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_4_ref_0);
//     let mut oncecell_9: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_0);
//     let mut oncecell_10: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     let mut lazy_0: crate::sync::Lazy<&str> = crate::sync::Lazy::default();
//     crate::sync::OnceCell::clone_from(oncecell_3_ref_0, oncecell_2_ref_0);
//     let mut bool_1: bool = crate::sync::OnceCell::eq(oncecell_1_ref_0, oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

}