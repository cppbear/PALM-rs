use core::panic::{RefUnwindSafe, UnwindSafe};

use critical_section::{CriticalSection, Mutex};
use portable_atomic::{AtomicBool, Ordering};

use crate::unsync;

pub(crate) struct OnceCell<T> {
    initialized: AtomicBool,
    // Use `unsync::OnceCell` internally since `Mutex` does not provide
    // interior mutability and to be able to re-use `get_or_try_init`.
    value: Mutex<unsync::OnceCell<T>>,
}

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
        OnceCell { initialized: AtomicBool::new(false), value: Mutex::new(unsync::OnceCell::new()) }
    }

    pub(crate) const fn with_value(value: T) -> OnceCell<T> {
        OnceCell {
            initialized: AtomicBool::new(true),
            value: Mutex::new(unsync::OnceCell::with_value(value)),
        }
    }

    #[inline]
    pub(crate) fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }

    #[cold]
    pub(crate) fn initialize<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        critical_section::with(|cs| {
            let cell = self.value.borrow(cs);
            cell.get_or_try_init(f).map(|_| {
                self.initialized.store(true, Ordering::Release);
            })
        })
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
        // SAFETY: The caller ensures that the value is initialized and access synchronized.
        self.value.borrow(CriticalSection::new()).get().unwrap_unchecked()
    }

    #[inline]
    pub(crate) fn get_mut(&mut self) -> Option<&mut T> {
        self.value.get_mut().get_mut()
    }

    #[inline]
    pub(crate) fn into_inner(self) -> Option<T> {
        self.value.into_inner().into_inner()
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::From;
	use std::ops::Drop;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut isize_0: isize = -18882isize;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_0);
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
    let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_1_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_1;
    let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
    let mut isize_1: isize = 12424isize;
    let mut oncebox_1: crate::race::once_box::OnceBox<f64> = crate::race::once_box::OnceBox::new();
    let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<f64> = &mut oncebox_1;
    let mut isize_2: isize = -2155isize;
    let mut isize_3: isize = -13660isize;
    let mut isize_4: isize = -18105isize;
    let mut oncebox_2: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_2_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_2;
    crate::race::once_box::OnceBox::drop(oncebox_2_ref_0);
    let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_4);
    let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_3);
    let mut oncecell_3_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_3;
    let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_3_ref_0, isize_2);
    let mut isize_5: &isize = std::result::Result::unwrap(result_0);
    let mut lazy_1: crate::unsync::Lazy<i8> = crate::unsync::Lazy::default();
    let mut oncebox_3: crate::race::once_box::OnceBox<f64> = crate::race::once_box::OnceBox::clone(oncebox_1_ref_0);
    let mut lazy_2: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_1);
    let mut oncebox_3_ref_0: &crate::race::once_box::OnceBox<f64> = &mut oncebox_3;
    let mut isize_6: &isize = crate::sync::OnceCell::wait(oncecell_2_ref_0);
    let mut result_1: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_0_ref_0, box_0);
    let mut bool_0: bool = crate::sync::OnceCell::eq(oncecell_1_ref_0, oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut isize_0: isize = -17552isize;
    let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_0);
    let mut isize_1: isize = -5916isize;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_1);
    let mut isize_2: isize = -12808isize;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
    let mut oncecell_1_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut tuple_0: () = ();
    let mut result_0: std::result::Result<(), std::boxed::Box<isize>> = std::result::Result::Ok(tuple_0);
    let mut oncecell_2: crate::unsync::OnceCell<std::result::Result<(), std::boxed::Box<isize>>> = crate::unsync::OnceCell::from(result_0);
    let mut oncecell_2_ref_0: &crate::unsync::OnceCell<std::result::Result<(), std::boxed::Box<isize>>> = &mut oncecell_2;
    let mut tuple_1: () = ();
    let mut oncecell_3: crate::unsync::OnceCell<()> = crate::unsync::OnceCell::with_value(tuple_1);
    let mut oncecell_4: crate::sync::OnceCell<std::boxed::Box<isize>> = crate::sync::OnceCell::new();
    let mut oncecell_5: crate::unsync::OnceCell<std::boxed::Box<isize>> = crate::unsync::OnceCell::default();
    let mut oncecell_5_ref_0: &crate::unsync::OnceCell<std::boxed::Box<isize>> = &mut oncecell_5;
    let mut isize_3: isize = 11129isize;
    let mut oncecell_6: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_6_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_6;
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut isize_4: isize = -6558isize;
    let mut lazy_1: crate::sync::Lazy<std::result::Result<&isize, isize>, isize> = crate::sync::Lazy::new(isize_4);
    let mut lazy_1_ref_0: &crate::sync::Lazy<std::result::Result<&isize, isize>, isize> = &mut lazy_1;
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
    let mut option_0: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
    let mut result_1: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_6_ref_0, isize_3);
    let mut isize_5: &isize = std::result::Result::unwrap(result_1);
    let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_1_ref_0);
    let mut lazy_2: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_2);
    let mut result_2: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
    let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_1;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut isize_0: isize = -15963isize;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
    let mut oncenonzerousize_0_ref_0: &crate::race::OnceNonZeroUsize = &mut oncenonzerousize_0;
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::new();
    let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
    let mut isize_1: isize = -8478isize;
    let mut isize_2: isize = -3816isize;
    let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_2);
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
    let mut isize_3: isize = 6252isize;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_3);
    let mut oncecell_3_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut isize_4: isize = -17087isize;
    let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_4_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_4;
    let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_4_ref_0, isize_4);
    let mut option_0: std::option::Option<&mut isize> = crate::sync::OnceCell::get_mut(oncecell_3_ref_0);
    let mut result_1: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_2_ref_0, isize_1);
    let mut tuple_0: () = std::result::Result::unwrap(result_1);
    let mut tuple_1: () = std::result::Result::unwrap(result_0);
    let mut result_2: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_1_ref_0, isize_0);
    let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool::new();
    let mut option_1: std::option::Option<isize> = crate::sync::OnceCell::into_inner(oncecell_0);
    let mut lazy_0: crate::sync::Lazy<u128> = crate::sync::Lazy::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut isize_0: isize = 7812isize;
    let mut isize_1: isize = -10516isize;
    let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
    let mut oncecell_0_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_0;
    let mut oncecell_1: crate::unsync::OnceCell<bool> = crate::unsync::OnceCell::new();
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<bool> = &mut oncecell_1;
    let mut oncecell_2: crate::unsync::OnceCell<bool> = crate::unsync::OnceCell::default();
    let mut oncecell_2_ref_0: &crate::unsync::OnceCell<bool> = &mut oncecell_2;
    let mut isize_2: isize = -14016isize;
    let mut lazy_0: crate::sync::Lazy<std::string::String, isize> = crate::sync::Lazy::new(isize_2);
    let mut lazy_0_ref_0: &crate::sync::Lazy<std::string::String, isize> = &mut lazy_0;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
    let mut oncebox_0_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut isize_3: isize = 15093isize;
    let mut isize_4: isize = -13415isize;
    let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_4);
    let mut oncecell_4_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_4;
    let mut isize_5: isize = -4878isize;
    let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_5_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_5;
    let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_5_ref_0, isize_5);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut oncecell_6: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
    let mut lazy_1: crate::sync::Lazy<i8> = crate::sync::Lazy::default();
    let mut result_1: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_4_ref_0, isize_3);
    crate::race::once_box::OnceBox::drop(oncebox_0_ref_0);
    let mut bool_0: bool = crate::unsync::OnceCell::eq(oncecell_2_ref_0, oncecell_1_ref_0);
    let mut result_2: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_0_ref_0, isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut oncecell_0: crate::sync::OnceCell<i32> = crate::sync::OnceCell::new();
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<i32> = &mut oncecell_0;
    let mut i32_0: i32 = -856i32;
    let mut oncecell_1: crate::sync::OnceCell<i32> = crate::sync::OnceCell::with_value(i32_0);
    let mut oncecell_1_ref_0: &crate::sync::OnceCell<i32> = &mut oncecell_1;
    let mut isize_0: isize = -6239isize;
    let mut isize_0_ref_0: &isize = &mut isize_0;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut u32_0: u32 = 3184u32;
    let mut oncecell_2: crate::unsync::OnceCell<u32> = crate::unsync::OnceCell::from(u32_0);
    let mut oncecell_2_ref_0: &crate::unsync::OnceCell<u32> = &mut oncecell_2;
    let mut oncecell_3: crate::unsync::OnceCell<u32> = crate::unsync::OnceCell::new();
    let mut oncecell_3_ref_0: &mut crate::unsync::OnceCell<u32> = &mut oncecell_3;
    let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_4_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_4;
    let mut isize_1: isize = -4468isize;
    let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_1);
    let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut isize_2: isize = 3136isize;
    let mut isize_2_ref_0: &isize = &mut isize_2;
    let mut onceref_1: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_1_ref_0: &crate::race::OnceRef<isize> = &mut onceref_1;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut result_0: std::result::Result<isize, isize> = crate::sync::Lazy::into_value(lazy_0);
    let mut option_0: std::option::Option<isize> = crate::sync::OnceCell::take(oncecell_4_ref_0);
    crate::unsync::OnceCell::clone_from(oncecell_3_ref_0, oncecell_2_ref_0);
    let mut result_1: std::result::Result<(), ()> = crate::race::OnceRef::set(onceref_0_ref_0, isize_0_ref_0);
    let mut bool_0: bool = crate::sync::OnceCell::eq(oncecell_1_ref_0, oncecell_0_ref_0);
    let mut result_2: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_0_ref_0, box_0);
    panic!("From RustyUnit with love");
}
}