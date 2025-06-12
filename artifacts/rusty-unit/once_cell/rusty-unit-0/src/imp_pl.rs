use std::{
    cell::UnsafeCell,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicU8, Ordering},
};

pub(crate) struct OnceCell<T> {
    state: AtomicU8,
    value: UnsafeCell<Option<T>>,
}

const INCOMPLETE: u8 = 0x0;
const RUNNING: u8 = 0x1;
const COMPLETE: u8 = 0x2;

// Why do we need `T: Send`?
// Thread A creates a `OnceCell` and shares it with
// scoped thread B, which fills the cell, which is
// then destroyed by A. That is, destructor observes
// a sent value.
unsafe impl<T: Sync + Send> Sync for OnceCell<T> {}
unsafe impl<T: Send> Send for OnceCell<T> {}

impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceCell<T> {}
impl<T: UnwindSafe> UnwindSafe for OnceCell<T> {}

impl<T> OnceCell<T> {
    pub(crate) const fn new() -> OnceCell<T> {
        OnceCell { state: AtomicU8::new(INCOMPLETE), value: UnsafeCell::new(None) }
    }

    pub(crate) const fn with_value(value: T) -> OnceCell<T> {
        OnceCell { state: AtomicU8::new(COMPLETE), value: UnsafeCell::new(Some(value)) }
    }

    /// Safety: synchronizes with store to value via Release/Acquire.
    #[inline]
    pub(crate) fn is_initialized(&self) -> bool {
        self.state.load(Ordering::Acquire) == COMPLETE
    }

    /// Safety: synchronizes with store to value via `is_initialized` or mutex
    /// lock/unlock, writes value only once because of the mutex.
    #[cold]
    pub(crate) fn initialize<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let mut f = Some(f);
        let mut res: Result<(), E> = Ok(());
        let slot: *mut Option<T> = self.value.get();
        initialize_inner(&self.state, &mut || {
            // We are calling user-supplied function and need to be careful.
            // - if it returns Err, we unlock mutex and return without touching anything
            // - if it panics, we unlock mutex and propagate panic without touching anything
            // - if it calls `set` or `get_or_try_init` re-entrantly, we get a deadlock on
            //   mutex, which is important for safety. We *could* detect this and panic,
            //   but that is more complicated
            // - finally, if it returns Ok, we store the value and store the flag with
            //   `Release`, which synchronizes with `Acquire`s.
            let f = unsafe { f.take().unwrap_unchecked() };
            match f() {
                Ok(value) => unsafe {
                    // Safe b/c we have a unique access and no panic may happen
                    // until the cell is marked as initialized.
                    debug_assert!((*slot).is_none());
                    *slot = Some(value);
                    true
                },
                Err(err) => {
                    res = Err(err);
                    false
                }
            }
        });
        res
    }

    #[cold]
    pub(crate) fn wait(&self) {
        let key = &self.state as *const _ as usize;
        unsafe {
            parking_lot_core::park(
                key,
                || self.state.load(Ordering::Acquire) != COMPLETE,
                || (),
                |_, _| (),
                parking_lot_core::DEFAULT_PARK_TOKEN,
                None,
            );
        }
    }

    /// Get the reference to the underlying value, without checking if the cell
    /// is initialized.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the cell is in initialized state, and that
    /// the contents are acquired by (synchronized to) this thread.
    pub(crate) unsafe fn get_unchecked(&self) -> &T {
        debug_assert!(self.is_initialized());
        let slot = &*self.value.get();
        slot.as_ref().unwrap_unchecked()
    }

    /// Gets the mutable reference to the underlying value.
    /// Returns `None` if the cell is empty.
    pub(crate) fn get_mut(&mut self) -> Option<&mut T> {
        // Safe b/c we have an exclusive access
        let slot: &mut Option<T> = unsafe { &mut *self.value.get() };
        slot.as_mut()
    }

    /// Consumes this `OnceCell`, returning the wrapped value.
    /// Returns `None` if the cell was empty.
    pub(crate) fn into_inner(self) -> Option<T> {
        self.value.into_inner()
    }
}

struct Guard<'a> {
    state: &'a AtomicU8,
    new_state: u8,
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        self.state.store(self.new_state, Ordering::Release);
        unsafe {
            let key = self.state as *const AtomicU8 as usize;
            parking_lot_core::unpark_all(key, parking_lot_core::DEFAULT_UNPARK_TOKEN);
        }
    }
}

// Note: this is intentionally monomorphic
#[inline(never)]
fn initialize_inner(state: &AtomicU8, init: &mut dyn FnMut() -> bool) {
    loop {
        let exchange =
            state.compare_exchange_weak(INCOMPLETE, RUNNING, Ordering::Acquire, Ordering::Acquire);
        match exchange {
            Ok(_) => {
                let mut guard = Guard { state, new_state: INCOMPLETE };
                if init() {
                    guard.new_state = COMPLETE;
                }
                return;
            }
            Err(COMPLETE) => return,
            Err(RUNNING) => unsafe {
                let key = state as *const AtomicU8 as usize;
                parking_lot_core::park(
                    key,
                    || state.load(Ordering::Relaxed) == RUNNING,
                    || (),
                    |_, _| (),
                    parking_lot_core::DEFAULT_PARK_TOKEN,
                    None,
                );
            },
            Err(INCOMPLETE) => (),
            Err(_) => debug_assert!(false),
        }
    }
}

#[test]
fn test_size() {
    use std::mem::size_of;

    assert_eq!(size_of::<OnceCell<bool>>(), 1 * size_of::<bool>() + size_of::<u8>());
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::convert::From;
	use std::ops::Drop;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut isize_0: isize = -5870isize;
    let mut isize_1: isize = -4329isize;
    let mut isize_2: isize = 3983isize;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut isize_3: isize = -6467isize;
    let mut isize_4: isize = -2537isize;
    let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
    let mut oncecell_0_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_0;
    let mut oncecell_1: crate::sync::OnceCell<bool> = crate::sync::OnceCell::default();
    let mut oncecell_1_ref_0: &crate::sync::OnceCell<bool> = &mut oncecell_1;
    let mut oncecell_2: crate::sync::OnceCell<bool> = crate::sync::OnceCell::default();
    let mut oncecell_2_ref_0: &mut crate::sync::OnceCell<bool> = &mut oncecell_2;
    let mut isize_5: isize = 1578isize;
    let mut isize_6: isize = 3641isize;
    let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_3;
    let mut lazy_0: crate::unsync::Lazy<u8> = crate::unsync::Lazy::default();
    let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_3_ref_0, isize_6);
    let mut isize_7: &isize = std::result::Result::unwrap(result_0);
    let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_5);
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::new();
    crate::sync::OnceCell::clone_from(oncecell_2_ref_0, oncecell_1_ref_0);
    let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_0_ref_0);
    let mut oncecell_6: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_4);
    let mut lazy_1: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_3);
    let mut oncecell_6_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_6;
    let mut result_1: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_6_ref_0, isize_2);
    let mut lazy_2: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_1);
    let mut oncecell_5_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_5;
    let mut result_2: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_5_ref_0, isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut isize_0: isize = -13078isize;
    let mut isize_1: isize = -14604isize;
    let mut isize_2: isize = 17686isize;
    let mut lazy_0: crate::sync::Lazy<std::boxed::Box<isize>, isize> = crate::sync::Lazy::new(isize_2);
    let mut lazy_0_ref_0: &crate::sync::Lazy<std::boxed::Box<isize>, isize> = &mut lazy_0;
    let mut isize_3: isize = 4929isize;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_3);
    let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut box_1: std::boxed::Box<isize> = std::boxed::Box::new();
    let mut box_2: std::boxed::Box<isize> = std::boxed::Box::new();
    let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_2);
    let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_1;
    let mut isize_4: isize = 23637isize;
    let mut isize_5: isize = -300isize;
    let mut lazy_1: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_5);
    let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut isize_6: isize = 1488isize;
    let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_6);
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut isize_7: &isize = crate::sync::OnceCell::wait(oncecell_3_ref_0);
    let mut result_0: std::result::Result<isize, isize> = crate::sync::Lazy::into_value(lazy_1);
    let mut oncecell_1_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_1;
    let mut result_1: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_1_ref_0, isize_4);
    let mut result_2: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_1_ref_0, box_1);
    let mut result_3: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_0_ref_0, box_0);
    let mut oncecell_2_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_2;
    let mut result_4: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_2_ref_0, isize_1);
    let mut lazy_2: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_0_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut isize_0: isize = 3729isize;
    let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_1_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_1;
    let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut isize_1: isize = 4738isize;
    let mut isize_2: isize = -2769isize;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_2);
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_4_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_4;
    let mut isize_3: isize = 8131isize;
    let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_3);
    let mut oncecell_6: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::default();
    let mut oncecell_6_ref_0: &crate::unsync::OnceCell<i128> = &mut oncecell_6;
    let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
    let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_1;
    let mut isize_4: isize = 14111isize;
    let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_4);
    let mut result_0: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
    let mut result_1: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_1_ref_0, box_0);
    let mut oncecell_7: crate::unsync::OnceCell<i128> = crate::unsync::OnceCell::clone(oncecell_6_ref_0);
    let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_4_ref_0);
    let mut result_2: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_3_ref_0, isize_1);
    let mut option_1: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_1_ref_0);
    let mut lazy_1: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_0);
    let mut isize_5: &isize = std::option::Option::unwrap(option_1);
    let mut option_2: std::option::Option<isize> = crate::sync::OnceCell::into_inner(oncecell_0);
    crate::race::once_box::OnceBox::drop(oncebox_0_ref_0);
    panic!("From RustyUnit with love");
}
}