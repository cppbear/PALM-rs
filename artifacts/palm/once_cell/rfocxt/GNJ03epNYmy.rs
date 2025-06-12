#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;
use atomic::{AtomicPtr, AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::ptr;
#[cfg(feature = "alloc")]
pub use self::once_box::OnceBox;
#[derive(Default, Debug)]
pub struct OnceBool {
    inner: OnceNonZeroUsize,
}
#[derive(Default, Debug)]
pub struct OnceNonZeroUsize {
    inner: AtomicUsize,
}
impl OnceBool {
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: OnceNonZeroUsize::new(),
        }
    }
    #[inline]
    pub fn get(&self) -> Option<bool> {}
    #[inline]
    pub fn set(&self, value: bool) -> Result<(), ()> {}
    pub fn get_or_init<F>(&self, f: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        Self::from_usize(self.inner.get_or_init(|| Self::to_usize(f())))
    }
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>
    where
        F: FnOnce() -> Result<bool, E>,
    {}
    #[inline]
    fn from_usize(value: NonZeroUsize) -> bool {
        value.get() == 1
    }
    #[inline]
    fn to_usize(value: bool) -> NonZeroUsize {}
}
impl OnceNonZeroUsize {
    #[inline]
    pub const fn new() -> Self {
        Self { inner: AtomicUsize::new(0) }
    }
    #[inline]
    pub fn get(&self) -> Option<NonZeroUsize> {}
    pub unsafe fn get_unchecked(&self) -> NonZeroUsize {}
    #[inline]
    pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {}
    pub fn get_or_init<F>(&self, f: F) -> NonZeroUsize
    where
        F: FnOnce() -> NonZeroUsize,
    {
        enum Void {}
        match self.get_or_try_init(|| Ok::<NonZeroUsize, Void>(f())) {
            Ok(val) => val,
            Err(void) => match void {}
        }
    }
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>
    where
        F: FnOnce() -> Result<NonZeroUsize, E>,
    {}
    #[cold]
    #[inline(never)]
    fn init<E>(
        &self,
        f: impl FnOnce() -> Result<NonZeroUsize, E>,
    ) -> Result<NonZeroUsize, E> {}
    #[inline(always)]
    fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {}
}
