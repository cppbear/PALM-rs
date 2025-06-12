// There's a lot of scary concurrent code in this module, but it is copied from
// `std::sync::Once` with two changes:
//   * no poisoning
//   * init function can fail

use std::{
    cell::{Cell, UnsafeCell},
    panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicBool, AtomicPtr, Ordering},
    thread::{self, Thread},
};

#[derive(Debug)]
pub(crate) struct OnceCell<T> {
    // This `queue` field is the core of the implementation. It encodes two
    // pieces of information:
    //
    // * The current state of the cell (`INCOMPLETE`, `RUNNING`, `COMPLETE`)
    // * Linked list of threads waiting for the current cell.
    //
    // State is encoded in two low bits. Only `INCOMPLETE` and `RUNNING` states
    // allow waiters.
    queue: AtomicPtr<Waiter>,
    value: UnsafeCell<Option<T>>,
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
        OnceCell { queue: AtomicPtr::new(INCOMPLETE_PTR), value: UnsafeCell::new(None) }
    }

    pub(crate) const fn with_value(value: T) -> OnceCell<T> {
        OnceCell { queue: AtomicPtr::new(COMPLETE_PTR), value: UnsafeCell::new(Some(value)) }
    }

    /// Safety: synchronizes with store to value via Release/(Acquire|SeqCst).
    #[inline]
    pub(crate) fn is_initialized(&self) -> bool {
        // An `Acquire` load is enough because that makes all the initialization
        // operations visible to us, and, this being a fast path, weaker
        // ordering helps with performance. This `Acquire` synchronizes with
        // `SeqCst` operations on the slow path.
        self.queue.load(Ordering::Acquire) == COMPLETE_PTR
    }

    /// Safety: synchronizes with store to value via SeqCst read from state,
    /// writes value only once because we never get to INCOMPLETE state after a
    /// successful write.
    #[cold]
    pub(crate) fn initialize<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let mut f = Some(f);
        let mut res: Result<(), E> = Ok(());
        let slot: *mut Option<T> = self.value.get();
        initialize_or_wait(
            &self.queue,
            Some(&mut || {
                let f = unsafe { f.take().unwrap_unchecked() };
                match f() {
                    Ok(value) => {
                        unsafe { *slot = Some(value) };
                        true
                    }
                    Err(err) => {
                        res = Err(err);
                        false
                    }
                }
            }),
        );
        res
    }

    #[cold]
    pub(crate) fn wait(&self) {
        initialize_or_wait(&self.queue, None);
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
        // Safe b/c we have a unique access.
        unsafe { &mut *self.value.get() }.as_mut()
    }

    /// Consumes this `OnceCell`, returning the wrapped value.
    /// Returns `None` if the cell was empty.
    #[inline]
    pub(crate) fn into_inner(self) -> Option<T> {
        // Because `into_inner` takes `self` by value, the compiler statically
        // verifies that it is not currently borrowed.
        // So, it is safe to move out `Option<T>`.
        self.value.into_inner()
    }
}

// Three states that a OnceCell can be in, encoded into the lower bits of `queue` in
// the OnceCell structure.
const INCOMPLETE: usize = 0x0;
const RUNNING: usize = 0x1;
const COMPLETE: usize = 0x2;
const INCOMPLETE_PTR: *mut Waiter = INCOMPLETE as *mut Waiter;
const COMPLETE_PTR: *mut Waiter = COMPLETE as *mut Waiter;

// Mask to learn about the state. All other bits are the queue of waiters if
// this is in the RUNNING state.
const STATE_MASK: usize = 0x3;

/// Representation of a node in the linked list of waiters in the RUNNING state.
/// A waiters is stored on the stack of the waiting threads.
#[repr(align(4))] // Ensure the two lower bits are free to use as state bits.
struct Waiter {
    thread: Cell<Option<Thread>>,
    signaled: AtomicBool,
    next: *mut Waiter,
}

/// Drains and notifies the queue of waiters on drop.
struct Guard<'a> {
    queue: &'a AtomicPtr<Waiter>,
    new_queue: *mut Waiter,
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        let queue = self.queue.swap(self.new_queue, Ordering::AcqRel);

        let state = strict::addr(queue) & STATE_MASK;
        assert_eq!(state, RUNNING);

        unsafe {
            let mut waiter = strict::map_addr(queue, |q| q & !STATE_MASK);
            while !waiter.is_null() {
                let next = (*waiter).next;
                let thread = (*waiter).thread.take().unwrap();
                (*waiter).signaled.store(true, Ordering::Release);
                waiter = next;
                thread.unpark();
            }
        }
    }
}

// Corresponds to `std::sync::Once::call_inner`.
//
// Originally copied from std, but since modified to remove poisoning and to
// support wait.
//
// Note: this is intentionally monomorphic
#[inline(never)]
fn initialize_or_wait(queue: &AtomicPtr<Waiter>, mut init: Option<&mut dyn FnMut() -> bool>) {
    let mut curr_queue = queue.load(Ordering::Acquire);

    loop {
        let curr_state = strict::addr(curr_queue) & STATE_MASK;
        match (curr_state, &mut init) {
            (COMPLETE, _) => return,
            (INCOMPLETE, Some(init)) => {
                let exchange = queue.compare_exchange(
                    curr_queue,
                    strict::map_addr(curr_queue, |q| (q & !STATE_MASK) | RUNNING),
                    Ordering::Acquire,
                    Ordering::Acquire,
                );
                if let Err(new_queue) = exchange {
                    curr_queue = new_queue;
                    continue;
                }
                let mut guard = Guard { queue, new_queue: INCOMPLETE_PTR };
                if init() {
                    guard.new_queue = COMPLETE_PTR;
                }
                return;
            }
            (INCOMPLETE, None) | (RUNNING, _) => {
                wait(queue, curr_queue);
                curr_queue = queue.load(Ordering::Acquire);
            }
            _ => debug_assert!(false),
        }
    }
}

fn wait(queue: &AtomicPtr<Waiter>, mut curr_queue: *mut Waiter) {
    let curr_state = strict::addr(curr_queue) & STATE_MASK;
    loop {
        let node = Waiter {
            thread: Cell::new(Some(thread::current())),
            signaled: AtomicBool::new(false),
            next: strict::map_addr(curr_queue, |q| q & !STATE_MASK),
        };
        let me = &node as *const Waiter as *mut Waiter;

        let exchange = queue.compare_exchange(
            curr_queue,
            strict::map_addr(me, |q| q | curr_state),
            Ordering::Release,
            Ordering::Relaxed,
        );
        if let Err(new_queue) = exchange {
            if strict::addr(new_queue) & STATE_MASK != curr_state {
                return;
            }
            curr_queue = new_queue;
            continue;
        }

        while !node.signaled.load(Ordering::Acquire) {
            thread::park();
        }
        break;
    }
}

// Polyfill of strict provenance from https://crates.io/crates/sptr.
//
// Use free-standing function rather than a trait to keep things simple and
// avoid any potential conflicts with future stabile std API.
mod strict {
    #[must_use]
    #[inline]
    pub(crate) fn addr<T>(ptr: *mut T) -> usize
    where
        T: Sized,
    {
        // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
        // SAFETY: Pointer-to-integer transmutes are valid (if you are okay with losing the
        // provenance).
        unsafe { core::mem::transmute(ptr) }
    }

    #[must_use]
    #[inline]
    pub(crate) fn with_addr<T>(ptr: *mut T, addr: usize) -> *mut T
    where
        T: Sized,
    {
        // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
        //
        // In the mean-time, this operation is defined to be "as if" it was
        // a wrapping_offset, so we can emulate it as such. This should properly
        // restore pointer provenance even under today's compiler.
        let self_addr = self::addr(ptr) as isize;
        let dest_addr = addr as isize;
        let offset = dest_addr.wrapping_sub(self_addr);

        // This is the canonical desugarring of this operation,
        // but `pointer::cast` was only stabilized in 1.38.
        // self.cast::<u8>().wrapping_offset(offset).cast::<T>()
        (ptr as *mut u8).wrapping_offset(offset) as *mut T
    }

    #[must_use]
    #[inline]
    pub(crate) fn map_addr<T>(ptr: *mut T, f: impl FnOnce(usize) -> usize) -> *mut T
    where
        T: Sized,
    {
        self::with_addr(ptr, f(addr(ptr)))
    }
}

// These test are snatched from std as well.
#[cfg(test)]
mod tests {
    use std::panic;
    use std::{sync::mpsc::channel, thread};

    use super::OnceCell;

    impl<T> OnceCell<T> {
        fn init(&self, f: impl FnOnce() -> T) {
            enum Void {}
            let _ = self.initialize(|| Ok::<T, Void>(f()));
        }
    }

    #[test]
    fn smoke_once() {
        static O: OnceCell<()> = OnceCell::new();
        let mut a = 0;
        O.init(|| a += 1);
        assert_eq!(a, 1);
        O.init(|| a += 1);
        assert_eq!(a, 1);
    }

    #[test]
    fn stampede_once() {
        static O: OnceCell<()> = OnceCell::new();
        static mut RUN: bool = false;

        let (tx, rx) = channel();
        for _ in 0..10 {
            let tx = tx.clone();
            thread::spawn(move || {
                for _ in 0..4 {
                    thread::yield_now()
                }
                unsafe {
                    O.init(|| {
                        assert!(!RUN);
                        RUN = true;
                    });
                    assert!(RUN);
                }
                tx.send(()).unwrap();
            });
        }

        unsafe {
            O.init(|| {
                assert!(!RUN);
                RUN = true;
            });
            assert!(RUN);
        }

        for _ in 0..10 {
            rx.recv().unwrap();
        }
    }

    #[test]
    fn poison_bad() {
        static O: OnceCell<()> = OnceCell::new();

        // poison the once
        let t = panic::catch_unwind(|| {
            O.init(|| panic!());
        });
        assert!(t.is_err());

        // we can subvert poisoning, however
        let mut called = false;
        O.init(|| {
            called = true;
        });
        assert!(called);

        // once any success happens, we stop propagating the poison
        O.init(|| {});
    }

    #[test]
    fn wait_for_force_to_finish() {
        static O: OnceCell<()> = OnceCell::new();

        // poison the once
        let t = panic::catch_unwind(|| {
            O.init(|| panic!());
        });
        assert!(t.is_err());

        // make sure someone's waiting inside the once via a force
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();
        let t1 = thread::spawn(move || {
            O.init(|| {
                tx1.send(()).unwrap();
                rx2.recv().unwrap();
            });
        });

        rx1.recv().unwrap();

        // put another waiter on the once
        let t2 = thread::spawn(|| {
            let mut called = false;
            O.init(|| {
                called = true;
            });
            assert!(!called);
        });

        tx2.send(()).unwrap();

        assert!(t1.join().is_ok());
        assert!(t2.join().is_ok());
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_size() {
        use std::mem::size_of;

        assert_eq!(size_of::<OnceCell<u32>>(), 4 * size_of::<u32>());
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
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_1() {
//     rusty_monitor::set_test_id(1);
//     let mut isize_0: isize = -1866isize;
//     let mut isize_1: isize = 4510isize;
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_1);
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut isize_2: isize = 4602isize;
//     let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_1;
//     let mut result_0: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_1_ref_0, isize_2);
//     let mut oncecell_2: crate::unsync::OnceCell<std::result::Result<(), isize>> = crate::unsync::OnceCell::from(result_0);
//     let mut oncecell_2_ref_0: &crate::unsync::OnceCell<std::result::Result<(), isize>> = &mut oncecell_2;
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut isize_3: isize = 12321isize;
//     let mut u128_0: u128 = 9281u128;
//     let mut oncecell_3: crate::unsync::OnceCell<u128> = crate::unsync::OnceCell::with_value(u128_0);
//     let mut oncecell_3_ref_0: &crate::unsync::OnceCell<u128> = &mut oncecell_3;
//     let mut isize_4: isize = -4892isize;
//     let mut isize_5: isize = 15022isize;
//     let mut oncecell_4: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_5);
//     let mut option_0: std::option::Option<isize> = crate::imp::OnceCell::into_inner(oncecell_4);
//     let mut isize_6: isize = std::option::Option::unwrap(option_0);
//     let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_4);
//     let mut oncecell_5: crate::unsync::OnceCell<u128> = crate::unsync::OnceCell::clone(oncecell_3_ref_0);
//     let mut lazy_1: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_3);
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
//     let mut oncecell_5_ref_0: &crate::unsync::OnceCell<u128> = &mut oncecell_5;
//     let mut option_1: std::option::Option<&u128> = crate::unsync::OnceCell::get(oncecell_5_ref_0);
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
//     let mut option_2: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
//     let mut result_1: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_0_ref_0, isize_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_2() {
//     rusty_monitor::set_test_id(2);
//     let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
//     let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut isize_0: isize = 25381isize;
//     let mut isize_1: isize = -3359isize;
//     let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_1);
//     let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut oncecell_3: crate::unsync::OnceCell<std::result::Result<isize, isize>> = crate::unsync::OnceCell::new();
//     let mut oncecell_3_ref_0: &crate::unsync::OnceCell<std::result::Result<isize, isize>> = &mut oncecell_3;
//     let mut isize_2: isize = 9725isize;
//     let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_2);
//     let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_5_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_5;
//     let mut isize_3: isize = 15651isize;
//     let mut isize_4: isize = -12104isize;
//     let mut oncecell_6: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_4);
//     let mut oncecell_6_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_6;
//     let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_6_ref_0, isize_3);
//     let mut oncecell_7: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_7_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_7;
//     let mut bool_0: bool = crate::unsync::OnceCell::eq(oncecell_7_ref_0, oncecell_5_ref_0);
//     let mut result_1: std::result::Result<isize, isize> = crate::sync::Lazy::into_value(lazy_0);
//     let mut tuple_0: () = std::result::Result::unwrap(result_0);
//     let mut lazy_1: crate::unsync::Lazy<&str> = crate::unsync::Lazy::default();
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
//     let mut result_2: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_2_ref_0, isize_0);
//     let mut isize_5: &isize = crate::sync::OnceCell::wait(oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut isize_0: isize = 15116isize;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_0);
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
    let mut isize_1: isize = -18661isize;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut isize_2: isize = -22782isize;
    let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut isize_3: isize = 17355isize;
    let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_3);
    let mut oncecell_3: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_3_ref_0: &mut crate::imp::OnceCell<isize> = &mut oncecell_3;
    let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut isize_4: isize = -13410isize;
    let mut lazy_1: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_4);
    let mut oncecell_6: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut result_0: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_1);
    let mut option_0: std::option::Option<&mut isize> = crate::imp::OnceCell::get_mut(oncecell_3_ref_0);
    let mut result_1: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
    let mut option_1: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
    let mut result_2: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_2_ref_0, isize_2);
    let mut oncecell_5_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_5;
    let mut option_2: std::option::Option<&isize> = crate::unsync::OnceCell::get(oncecell_5_ref_0);
    let mut isize_5: &isize = std::result::Result::unwrap(result_2);
    let mut oncecell_4_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_4;
    let mut bool_0: bool = crate::unsync::OnceCell::eq(oncecell_4_ref_0, oncecell_1_ref_0);
    let mut option_3: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_5() {
//     rusty_monitor::set_test_id(5);
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
//     let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
//     let mut isize_0: isize = 17902isize;
//     let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut bool_0: bool = false;
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut oncecell_1: crate::unsync::OnceCell<std::result::Result<isize, isize>> = crate::unsync::OnceCell::new();
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<std::result::Result<isize, isize>> = &mut oncecell_1;
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_2;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool::new();
//     let mut oncebool_1_ref_0: &crate::race::OnceBool = &mut oncebool_1;
//     let mut isize_1: isize = -16821isize;
//     let mut isize_1_ref_0: &isize = &mut isize_1;
//     let mut onceref_1: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
//     let mut onceref_1_ref_0: &crate::race::OnceRef<isize> = &mut onceref_1;
//     let mut option_0: std::option::Option<bool> = crate::race::OnceBool::get(oncebool_1_ref_0);
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
//     let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_2_ref_0);
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut option_2: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
//     let mut isize_2: &isize = std::option::Option::unwrap(option_2);
//     let mut isize_3: isize = std::option::Option::unwrap(option_1);
//     let mut result_0: std::result::Result<(), ()> = crate::race::OnceBool::set(oncebool_0_ref_0, bool_0);
//     let mut option_3: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_0);
//     let mut onceref_2: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
//     let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_0);
//     let mut oncecell_4: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
//     let mut option_4: std::option::Option<&isize> = crate::race::OnceRef::get(onceref_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_7() {
//     rusty_monitor::set_test_id(7);
//     let mut oncecell_0: crate::unsync::OnceCell<i16> = crate::unsync::OnceCell::new();
//     let mut oncecell_0_ref_0: &crate::unsync::OnceCell<i16> = &mut oncecell_0;
//     let mut i16_0: i16 = 16295i16;
//     let mut oncecell_1: crate::unsync::OnceCell<i16> = crate::unsync::OnceCell::from(i16_0);
//     let mut oncecell_1_ref_0: &crate::unsync::OnceCell<i16> = &mut oncecell_1;
//     let mut isize_0: isize = -607isize;
//     let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_0);
//     let mut oncecell_2_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_2;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut u128_0: u128 = 5471u128;
//     let mut oncecell_3: crate::sync::OnceCell<u128> = crate::sync::OnceCell::from(u128_0);
//     let mut oncecell_3_ref_0: &crate::sync::OnceCell<u128> = &mut oncecell_3;
//     let mut u128_1: u128 = 147u128;
//     let mut oncecell_4: crate::sync::OnceCell<u128> = crate::sync::OnceCell::with_value(u128_1);
//     let mut oncecell_4_ref_0: &crate::sync::OnceCell<u128> = &mut oncecell_4;
//     let mut oncecell_5: crate::sync::OnceCell<usize> = crate::sync::OnceCell::default();
//     let mut oncecell_5_ref_0: &crate::sync::OnceCell<usize> = &mut oncecell_5;
//     let mut usize_0: usize = 9186usize;
//     let mut oncecell_6: crate::sync::OnceCell<usize> = crate::sync::OnceCell::with_value(usize_0);
//     let mut oncecell_6_ref_0: &mut crate::sync::OnceCell<usize> = &mut oncecell_6;
//     let mut isize_1: isize = 14598isize;
//     let mut oncecell_7: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
//     let mut oncecell_7_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_7;
//     let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_7_ref_0);
//     crate::sync::OnceCell::clone_from(oncecell_6_ref_0, oncecell_5_ref_0);
//     let mut bool_0: bool = crate::sync::OnceCell::eq(oncecell_4_ref_0, oncecell_3_ref_0);
//     let mut option_1: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
//     let mut option_2: std::option::Option<&mut isize> = crate::sync::OnceCell::get_mut(oncecell_2_ref_0);
//     let mut bool_1: bool = crate::unsync::OnceCell::eq(oncecell_1_ref_0, oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_8() {
//     rusty_monitor::set_test_id(8);
//     let mut oncecell_0: crate::sync::OnceCell<dyn std::ops::FnMut> = crate::sync::OnceCell::new();
//     let mut oncecell_0_ref_0: &mut crate::sync::OnceCell<dyn std::ops::FnMut> = &mut oncecell_0;
//     let mut option_0: std::option::Option<&mut dyn std::ops::FnMut> = crate::sync::OnceCell::get_mut(oncecell_0_ref_0);
//     let mut str_0: &str = "ih0V";
//     let mut oncecell_1: crate::sync::OnceCell<&str> = crate::sync::OnceCell::from(str_0);
//     let mut oncecell_1_ref_0: &crate::sync::OnceCell<&str> = &mut oncecell_1;
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
//     let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
//     let mut isize_0: isize = -2811isize;
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_2_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_2;
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
//     let mut isize_1: isize = -20111isize;
//     let mut oncecell_4: crate::imp::OnceCell<std::string::String> = crate::imp::OnceCell::new();
//     let mut oncecell_4_ref_0: &crate::imp::OnceCell<std::string::String> = &mut oncecell_4;
//     let mut isize_2: isize = 9981isize;
//     let mut oncecell_5: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_2);
//     let mut oncecell_5_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_5;
//     crate::imp::OnceCell::wait(oncecell_5_ref_0);
//     let mut oncecell_6: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_6_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_6;
//     let mut result_0: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_6_ref_0, isize_1);
//     let mut lazy_0: crate::unsync::Lazy<u32> = crate::unsync::Lazy::default();
//     let mut option_1: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_3_ref_0);
//     let mut result_1: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_2_ref_0, isize_0);
//     let mut oncecell_7: crate::sync::OnceCell<&str> = crate::sync::OnceCell::clone(oncecell_1_ref_0);
//     let mut oncecell_7_ref_0: &crate::sync::OnceCell<&str> = &mut oncecell_7;
//     let mut str_1: &str = crate::sync::OnceCell::wait(oncecell_7_ref_0);
//     let mut tuple_0: () = std::result::Result::unwrap(result_0);
//     let mut tuple_1: () = std::result::Result::unwrap(result_1);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_1_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut isize_0: isize = 3264isize;
    let mut isize_1: isize = 18079isize;
    let mut lazy_0: crate::sync::Lazy<std::result::Result<isize, isize>, isize> = crate::sync::Lazy::new(isize_1);
    let mut lazy_0_ref_0: &crate::sync::Lazy<std::result::Result<isize, isize>, isize> = &mut lazy_0;
    let mut isize_2: isize = 20630isize;
    let mut str_0: &str = "rufPknL";
    let mut oncecell_2: crate::unsync::OnceCell<&str> = crate::unsync::OnceCell::from(str_0);
    let mut oncecell_2_ref_0: &crate::unsync::OnceCell<&str> = &mut oncecell_2;
    let mut isize_3: isize = -915isize;
    let mut isize_4: isize = -2458isize;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_4);
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut isize_5: isize = -9886isize;
    let mut isize_6: isize = 5220isize;
    let mut isize_7: isize = 6601isize;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut lazy_1: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_7);
    let mut onceref_1: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
    let mut oncecell_4: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_6);
    let mut onceref_2: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_3: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
    let mut oncecell_5: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_5);
    let mut result_0: std::result::Result<&isize, (&isize, isize)> = crate::sync::OnceCell::try_insert(oncecell_3_ref_0, isize_3);
    let mut oncecell_6: crate::unsync::OnceCell<&str> = crate::unsync::OnceCell::clone(oncecell_2_ref_0);
    let mut lazy_2: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_2);
    let mut oncecell_7: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_0);
    let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_1_ref_0);
    let mut option_1: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_12() {
//     rusty_monitor::set_test_id(12);
//     let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_0_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_0;
//     let mut oncecell_1: crate::sync::OnceCell<std::result::Result<(), isize>> = crate::sync::OnceCell::new();
//     let mut oncecell_1_ref_0: &crate::sync::OnceCell<std::result::Result<(), isize>> = &mut oncecell_1;
//     let mut isize_0: isize = 12516isize;
//     let mut isize_1: isize = 15109isize;
//     let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_1);
//     let mut oncecell_2_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_2;
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
//     let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_3_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_3;
//     let mut isize_2: isize = -16808isize;
//     let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
//     let mut oncecell_4_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_4;
//     let mut box_1: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_1);
//     let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_1;
//     let mut lazy_0: crate::unsync::Lazy<f64> = crate::unsync::Lazy::default();
//     let mut result_0: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_4_ref_0, isize_2);
//     let mut option_0: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_3_ref_0);
//     let mut result_1: std::result::Result<(), std::boxed::Box<isize>> = crate::race::once_box::OnceBox::set(oncebox_0_ref_0, box_0);
//     let mut tuple_0: () = std::result::Result::unwrap(result_0);
//     let mut result_2: std::result::Result<&isize, (&isize, isize)> = crate::unsync::OnceCell::try_insert(oncecell_2_ref_0, isize_0);
//     let mut tuple_1: () = std::result::Result::unwrap(result_1);
//     let mut isize_3: &isize = std::result::Result::unwrap(result_2);
//     let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut isize_0: isize = 2229isize;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut oncenonzerousize_0_ref_0: &crate::race::OnceNonZeroUsize = &mut oncenonzerousize_0;
    let mut isize_1: isize = -453isize;
    let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_1);
    let mut result_0: std::result::Result<isize, isize> = crate::sync::Lazy::into_value(lazy_0);
    let mut oncecell_0: crate::imp::OnceCell<std::result::Result<isize, isize>> = crate::imp::OnceCell::with_value(result_0);
    let mut oncecell_0_ref_0: &crate::imp::OnceCell<std::result::Result<isize, isize>> = &mut oncecell_0;
    let mut isize_2: isize = 5384isize;
    let mut isize_3: isize = -803isize;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_3);
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut isize_4: isize = -154isize;
    let mut isize_4_ref_0: &isize = &mut isize_4;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut isize_5: isize = 3267isize;
    let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_5);
    let mut oncecell_2_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_2;
    let mut onceref_1: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_1_ref_0: &crate::race::OnceRef<isize> = &mut onceref_1;
    let mut option_0: std::option::Option<&isize> = crate::race::OnceRef::get(onceref_1_ref_0);
    let mut isize_6: &isize = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<&isize> = crate::unsync::OnceCell::get(oncecell_2_ref_0);
    let mut result_1: std::result::Result<(), ()> = crate::race::OnceRef::set(onceref_0_ref_0, isize_4_ref_0);
    let mut result_2: std::result::Result<(), isize> = crate::unsync::OnceCell::set(oncecell_1_ref_0, isize_2);
    let mut isize_7: &isize = std::option::Option::unwrap(option_1);
    let mut oncecell_3: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_3;
    crate::imp::OnceCell::wait(oncecell_3_ref_0);
    let mut oncecell_4: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_25() {
//     rusty_monitor::set_test_id(25);
//     let mut oncecell_0: crate::imp::OnceCell<std::result::Result<(), isize>> = crate::imp::OnceCell::new();
//     let mut oncecell_0_ref_0: &crate::imp::OnceCell<std::result::Result<(), isize>> = &mut oncecell_0;
//     let mut isize_0: isize = -2199isize;
//     let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_0);
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut oncecell_1: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
//     let mut oncecell_1_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_1;
//     let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
//     let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
//     let mut oncecell_3_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_3;
//     let mut box_0: std::boxed::Box<isize> = std::boxed::Box::new();
//     let mut isize_1: isize = 3303isize;
//     let mut isize_1_ref_0: &isize = &mut isize_1;
//     let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
//     let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
//     let mut oncecell_4: crate::unsync::OnceCell<f32> = crate::unsync::OnceCell::new();
//     let mut oncecell_4_ref_0: &crate::unsync::OnceCell<f32> = &mut oncecell_4;
//     let mut f32_0: f32 = 8507.040192f32;
//     let mut oncecell_5: crate::unsync::OnceCell<f32> = crate::unsync::OnceCell::from(f32_0);
//     let mut oncecell_5_ref_0: &mut crate::unsync::OnceCell<f32> = &mut oncecell_5;
//     crate::unsync::OnceCell::clone_from(oncecell_5_ref_0, oncecell_4_ref_0);
//     let mut result_0: std::result::Result<(), ()> = crate::race::OnceRef::set(onceref_0_ref_0, isize_1_ref_0);
//     let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::with_value(box_0);
//     let mut oncecell_6: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
//     crate::sync::OnceCell::clone_from(oncecell_3_ref_0, oncecell_2_ref_0);
//     let mut oncebox_2: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
//     crate::imp::OnceCell::wait(oncecell_1_ref_0);
//     let mut result_1: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_27() {
//     rusty_monitor::set_test_id(27);
//     let mut u32_0: u32 = 9826u32;
//     let mut oncecell_0: crate::sync::OnceCell<u32> = crate::sync::OnceCell::with_value(u32_0);
//     let mut oncecell_0_ref_0: &crate::sync::OnceCell<u32> = &mut oncecell_0;
//     let mut usize_0: usize = 3571usize;
//     let mut oncecell_1: crate::sync::OnceCell<usize> = crate::sync::OnceCell::from(usize_0);
//     let mut oncecell_1_ref_0: &crate::sync::OnceCell<usize> = &mut oncecell_1;
//     let mut usize_1: usize = 9859usize;
//     let mut oncecell_2: crate::sync::OnceCell<usize> = crate::sync::OnceCell::from(usize_1);
//     let mut oncecell_2_ref_0: &mut crate::sync::OnceCell<usize> = &mut oncecell_2;
//     let mut isize_0: isize = 8150isize;
//     let mut oncecell_3: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_0);
//     let mut oncecell_3_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_3;
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_0_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut oncecell_4: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
//     let mut oncecell_4_ref_0: &mut crate::imp::OnceCell<isize> = &mut oncecell_4;
//     let mut oncecell_5: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
//     let mut oncecell_6: crate::sync::OnceCell<f64> = crate::sync::OnceCell::default();
//     let mut oncecell_6_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_6;
//     let mut f64_0: f64 = -5577.830555f64;
//     let mut oncecell_7: crate::sync::OnceCell<f64> = crate::sync::OnceCell::with_value(f64_0);
//     let mut oncecell_7_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_7;
//     let mut oncecell_8: crate::sync::OnceCell<f64> = crate::sync::OnceCell::clone(oncecell_7_ref_0);
//     let mut oncecell_8_ref_0: &mut crate::sync::OnceCell<f64> = &mut oncecell_8;
//     crate::sync::OnceCell::clone_from(oncecell_8_ref_0, oncecell_6_ref_0);
//     let mut option_0: std::option::Option<isize> = crate::imp::OnceCell::into_inner(oncecell_5);
//     let mut option_1: std::option::Option<&mut isize> = crate::imp::OnceCell::get_mut(oncecell_4_ref_0);
//     crate::race::once_box::OnceBox::drop(oncebox_0_ref_0);
//     let mut isize_1: &mut isize = std::option::Option::unwrap(option_1);
//     let mut bool_0: bool = crate::imp::OnceCell::is_initialized(oncecell_3_ref_0);
//     let mut isize_2: isize = std::option::Option::unwrap(option_0);
//     crate::sync::OnceCell::clone_from(oncecell_2_ref_0, oncecell_1_ref_0);
//     let mut oncecell_9: crate::sync::OnceCell<u32> = crate::sync::OnceCell::clone(oncecell_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut bool_0: bool = true;
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
    let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
    let mut oncecell_1: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
    let mut isize_0: isize = 14218isize;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut isize_1: isize = 3593isize;
    let mut oncecell_4: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_1);
    let mut oncecell_4_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_4;
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut isize_2: isize = -2289isize;
    let mut isize_3: isize = 4538isize;
    let mut lazy_0: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_3);
    let mut result_0: std::result::Result<isize, isize> = crate::unsync::Lazy::into_value(lazy_0);
    let mut lazy_1: crate::sync::Lazy<usize> = crate::sync::Lazy::default();
    let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_2);
    let mut option_0: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
    let mut bool_1: bool = crate::imp::OnceCell::is_initialized(oncecell_4_ref_0);
    let mut result_1: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_3_ref_0, isize_0);
    let mut oncecell_5_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_5;
    crate::sync::OnceCell::clone_from(oncecell_5_ref_0, oncecell_2_ref_0);
    let mut option_1: std::option::Option<isize> = crate::imp::OnceCell::into_inner(oncecell_1);
    let mut option_2: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_0_ref_0);
    let mut result_2: std::result::Result<(), ()> = crate::race::OnceBool::set(oncebool_0_ref_0, bool_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut oncebox_0: crate::race::once_box::OnceBox<u64> = crate::race::once_box::OnceBox::new();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<u64> = &mut oncebox_0;
    let mut oncecell_0: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_0_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_0;
    let mut isize_0: isize = -19235isize;
    let mut isize_1: isize = -4660isize;
    let mut oncecell_1: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_1);
    let mut oncecell_1_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_1;
    let mut isize_2: isize = 642isize;
    let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_2);
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_2;
    let mut oncecell_3: crate::imp::OnceCell<std::result::Result<isize, isize>> = crate::imp::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::imp::OnceCell<std::result::Result<isize, isize>> = &mut oncecell_3;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut oncenonzerousize_0_ref_0: &crate::race::OnceNonZeroUsize = &mut oncenonzerousize_0;
    let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_4_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_4;
    let mut isize_3: isize = -8757isize;
    let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_3);
    let mut oncecell_5_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_5;
    let mut bool_0: bool = crate::unsync::OnceCell::eq(oncecell_5_ref_0, oncecell_4_ref_0);
    let mut lazy_0: crate::unsync::Lazy<i32> = crate::unsync::Lazy::default();
    let mut option_0: std::option::Option<&isize> = crate::sync::OnceCell::get(oncecell_2_ref_0);
    let mut isize_4: &isize = std::option::Option::unwrap(option_0);
    let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_1_ref_0, isize_0);
    let mut bool_1: bool = crate::imp::OnceCell::is_initialized(oncecell_0_ref_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut oncebox_1: crate::race::once_box::OnceBox<u64> = crate::race::once_box::OnceBox::clone(oncebox_0_ref_0);
    let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<u64> = &mut oncebox_1;
    let mut oncebox_2: crate::race::once_box::OnceBox<u64> = crate::race::once_box::OnceBox::clone(oncebox_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut oncecell_0: crate::sync::OnceCell<f64> = crate::sync::OnceCell::new();
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_0;
    let mut oncecell_1: crate::sync::OnceCell<f64> = crate::sync::OnceCell::default();
    let mut oncecell_1_ref_0: &crate::sync::OnceCell<f64> = &mut oncecell_1;
    let mut isize_0: isize = 6012isize;
    let mut isize_1: isize = 4378isize;
    let mut isize_1_ref_0: &isize = &mut isize_1;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::default();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut isize_2: isize = 14713isize;
    let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_2);
    let mut isize_3: isize = 621isize;
    let mut isize_4: isize = 9568isize;
    let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_4);
    let mut oncecell_3_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_3;
    let mut isize_5: isize = -7080isize;
    let mut oncecell_4: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_5);
    let mut oncecell_4_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_4;
    let mut isize_6: isize = -9093isize;
    let mut lazy_0: crate::sync::Lazy<std::result::Result<isize, isize>, isize> = crate::sync::Lazy::new(isize_6);
    let mut lazy_0_ref_0: &crate::sync::Lazy<std::result::Result<isize, isize>, isize> = &mut lazy_0;
    let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_4_ref_0);
    let mut option_1: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_3_ref_0);
    let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut lazy_1: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_3);
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::new();
    let mut lazy_2: crate::sync::Lazy<i16> = crate::sync::Lazy::default();
    let mut oncecell_6: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut result_0: std::result::Result<(), ()> = crate::race::OnceRef::set(onceref_0_ref_0, isize_1_ref_0);
    let mut oncecell_7: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_0);
    let mut bool_0: bool = crate::sync::OnceCell::eq(oncecell_1_ref_0, oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_40() {
//     rusty_monitor::set_test_id(40);
//     let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::default();
//     let mut isize_0: isize = -17657isize;
//     let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_0);
//     let mut oncecell_1_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_1;
//     let mut oncecell_2: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
//     let mut oncecell_2_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_2;
//     let mut isize_1: isize = 19199isize;
//     let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
//     let mut oncecell_3_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_3;
//     let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
//     let mut oncebox_0_ref_0: &mut crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
//     let mut isize_2: isize = -7223isize;
//     let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
//     let mut oncecell_4_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_4;
//     let mut isize_3: isize = -1787isize;
//     let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
//     let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
//     let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut lazy_0: crate::unsync::Lazy<&str> = crate::unsync::Lazy::default();
//     let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_3);
//     let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_4_ref_0, isize_2);
//     crate::race::once_box::OnceBox::drop(oncebox_0_ref_0);
//     let mut oncecell_5_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_5;
//     crate::unsync::OnceCell::clone_from(oncecell_5_ref_0, oncecell_3_ref_0);
//     crate::imp::OnceCell::wait(oncecell_2_ref_0);
//     let mut option_0: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_1_ref_0);
//     let mut oncenonzerousize_1: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
//     let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_0);
//     let mut isize_4: isize = std::option::Option::unwrap(option_1);
//     let mut isize_5: isize = std::option::Option::unwrap(option_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut isize_0: isize = 2430isize;
    let mut lazy_0: crate::unsync::Lazy<std::result::Result<(), ()>, isize> = crate::unsync::Lazy::new(isize_0);
    let mut lazy_0_ref_0: &crate::unsync::Lazy<std::result::Result<(), ()>, isize> = &mut lazy_0;
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut isize_1: isize = -3029isize;
    let mut oncebox_1: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::default();
    let mut oncebox_1_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_1;
    let mut isize_2: isize = 2064isize;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_2);
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_0;
    let mut bool_0: bool = false;
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
    let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
    let mut isize_3: isize = -7049isize;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_3);
    let mut isize_4: isize = 9220isize;
    let mut isize_5: isize = -2801isize;
    let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut option_0: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_2);
    let mut lazy_1: crate::unsync::Lazy<isize, isize> = crate::unsync::Lazy::new(isize_5);
    let mut oncecell_3: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_4);
    let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_1);
    let mut oncebool_1: crate::race::OnceBool = crate::race::OnceBool::new();
    let mut result_0: std::result::Result<(), ()> = crate::race::OnceBool::set(oncebool_0_ref_0, bool_0);
    let mut isize_6: &isize = crate::sync::OnceCell::wait(oncecell_0_ref_0);
    let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
    let mut option_2: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
    let mut lazy_2: crate::unsync::Lazy<u16> = crate::unsync::Lazy::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut isize_0: isize = -2751isize;
    let mut isize_1: isize = 1403isize;
    let mut oncecell_0: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
    let mut oncecell_0_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_0;
    let mut isize_2: isize = -8902isize;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_2);
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut oncecell_2: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut bool_0: bool = true;
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::new();
    let mut oncebool_0_ref_0: &crate::race::OnceBool = &mut oncebool_0;
    let mut oncecell_3: crate::unsync::OnceCell<i32> = crate::unsync::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::unsync::OnceCell<i32> = &mut oncecell_3;
    let mut oncecell_4: crate::unsync::OnceCell<i32> = crate::unsync::OnceCell::default();
    let mut oncecell_4_ref_0: &crate::unsync::OnceCell<i32> = &mut oncecell_4;
    let mut isize_3: isize = -2067isize;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut isize_4: isize = -695isize;
    let mut lazy_0: crate::unsync::Lazy<std::result::Result<(), isize>, isize> = crate::unsync::Lazy::new(isize_4);
    let mut lazy_0_ref_0: &crate::unsync::Lazy<std::result::Result<(), isize>, isize> = &mut lazy_0;
    let mut oncecell_5: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_6: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut option_0: std::option::Option<&isize> = crate::race::OnceRef::get(onceref_0_ref_0);
    let mut oncecell_7: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_3);
    let mut bool_1: bool = crate::unsync::OnceCell::eq(oncecell_4_ref_0, oncecell_3_ref_0);
    let mut result_0: std::result::Result<(), ()> = crate::race::OnceBool::set(oncebool_0_ref_0, bool_0);
    let mut option_1: std::option::Option<isize> = crate::sync::OnceCell::into_inner(oncecell_2);
    let mut bool_2: bool = crate::unsync::OnceCell::eq(oncecell_1_ref_0, oncecell_0_ref_0);
    let mut oncecell_8: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut isize_0: isize = -1234isize;
    let mut oncecell_0: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_0);
    let mut oncecell_0_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_0;
    let mut oncecell_1: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_1_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_1;
    let mut isize_1: isize = 1699isize;
    let mut oncecell_2: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_1);
    let mut isize_2: isize = 6807isize;
    let mut oncecell_3: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_2);
    let mut isize_3: isize = 736isize;
    let mut oncecell_4: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_4_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_4;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::new();
    let mut oncenonzerousize_0_ref_0: &crate::race::OnceNonZeroUsize = &mut oncenonzerousize_0;
    let mut f64_0: f64 = 7800.250365f64;
    let mut oncecell_5: crate::unsync::OnceCell<f64> = crate::unsync::OnceCell::with_value(f64_0);
    let mut oncecell_5_ref_0: &crate::unsync::OnceCell<f64> = &mut oncecell_5;
    let mut oncecell_6: crate::unsync::OnceCell<f64> = crate::unsync::OnceCell::new();
    let mut oncecell_6_ref_0: &mut crate::unsync::OnceCell<f64> = &mut oncecell_6;
    let mut isize_4: isize = -562isize;
    let mut oncecell_7: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_4);
    let mut oncecell_7_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_7;
    let mut bool_0: bool = crate::imp::OnceCell::is_initialized(oncecell_7_ref_0);
    crate::unsync::OnceCell::clone_from(oncecell_6_ref_0, oncecell_5_ref_0);
    let mut option_0: std::option::Option<isize> = crate::sync::OnceCell::take(oncecell_4_ref_0);
    let mut oncecell_8: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_3);
    let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::into_inner(oncecell_3);
    let mut oncecell_9: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut option_2: std::option::Option<isize> = crate::imp::OnceCell::into_inner(oncecell_2);
    let mut option_3: std::option::Option<isize> = crate::sync::OnceCell::into_inner(oncecell_9);
    let mut option_4: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_1_ref_0);
    let mut isize_5: isize = std::option::Option::unwrap(option_0);
    let mut bool_1: bool = crate::imp::OnceCell::is_initialized(oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut oncebox_0: crate::race::once_box::OnceBox<isize> = crate::race::once_box::OnceBox::new();
    let mut oncebox_0_ref_0: &crate::race::once_box::OnceBox<isize> = &mut oncebox_0;
    let mut oncecell_0: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_0_ref_0: &mut crate::sync::OnceCell<isize> = &mut oncecell_0;
    let mut isize_0: isize = -8099isize;
    let mut lazy_0: crate::sync::Lazy<bool, isize> = crate::sync::Lazy::new(isize_0);
    let mut result_0: std::result::Result<bool, isize> = crate::sync::Lazy::into_value(lazy_0);
    let mut oncecell_1: crate::unsync::OnceCell<std::result::Result<bool, isize>> = crate::unsync::OnceCell::from(result_0);
    let mut oncecell_1_ref_0: &crate::unsync::OnceCell<std::result::Result<bool, isize>> = &mut oncecell_1;
    let mut oncecell_2: crate::sync::OnceCell<i16> = crate::sync::OnceCell::new();
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<i16> = &mut oncecell_2;
    let mut oncecell_3: crate::sync::OnceCell<i16> = crate::sync::OnceCell::new();
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<i16> = &mut oncecell_3;
    let mut isize_1: isize = 5333isize;
    let mut f32_0: f32 = 11934.153496f32;
    let mut oncecell_4: crate::unsync::OnceCell<f32> = crate::unsync::OnceCell::with_value(f32_0);
    let mut oncecell_4_ref_0: &crate::unsync::OnceCell<f32> = &mut oncecell_4;
    let mut onceref_0: crate::race::OnceRef<isize> = crate::race::OnceRef::new();
    let mut onceref_0_ref_0: &crate::race::OnceRef<isize> = &mut onceref_0;
    let mut oncenonzerousize_0: crate::race::OnceNonZeroUsize = crate::race::OnceNonZeroUsize::default();
    let mut oncenonzerousize_0_ref_0: &crate::race::OnceNonZeroUsize = &mut oncenonzerousize_0;
    let mut oncecell_5: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::new();
    let mut oncecell_6: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncebool_0: crate::race::OnceBool = crate::race::OnceBool::default();
    let mut oncecell_7: crate::unsync::OnceCell<f32> = crate::unsync::OnceCell::clone(oncecell_4_ref_0);
    let mut oncecell_8: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_1);
    let mut bool_0: bool = crate::sync::OnceCell::eq(oncecell_3_ref_0, oncecell_2_ref_0);
    let mut option_0: std::option::Option<&mut isize> = crate::sync::OnceCell::get_mut(oncecell_0_ref_0);
    let mut oncecell_9: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut option_1: std::option::Option<&isize> = crate::race::once_box::OnceBox::get(oncebox_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut i128_0: i128 = -7738i128;
    let mut oncecell_0: crate::sync::OnceCell<i128> = crate::sync::OnceCell::with_value(i128_0);
    let mut oncecell_0_ref_0: &crate::sync::OnceCell<i128> = &mut oncecell_0;
    let mut oncecell_1: crate::sync::OnceCell<i128> = crate::sync::OnceCell::default();
    let mut oncecell_1_ref_0: &mut crate::sync::OnceCell<i128> = &mut oncecell_1;
    let mut isize_0: isize = 2192isize;
    let mut isize_1: isize = -4298isize;
    let mut oncecell_2: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
    let mut oncecell_2_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_2;
    let mut isize_2: isize = -1613isize;
    let mut isize_3: isize = -2289isize;
    let mut oncecell_3: crate::sync::OnceCell<isize> = crate::sync::OnceCell::from(isize_3);
    let mut oncecell_3_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_3;
    let mut oncecell_4: crate::unsync::OnceCell<u128> = crate::unsync::OnceCell::default();
    let mut oncecell_4_ref_0: &crate::unsync::OnceCell<u128> = &mut oncecell_4;
    let mut isize_4: isize = 3208isize;
    let mut oncecell_5: crate::sync::OnceCell<isize> = crate::sync::OnceCell::default();
    let mut oncecell_5_ref_0: &crate::sync::OnceCell<isize> = &mut oncecell_5;
    let mut isize_5: isize = -12393isize;
    let mut oncecell_6: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_5);
    let mut oncecell_6_ref_0: &mut crate::unsync::OnceCell<isize> = &mut oncecell_6;
    let mut option_0: std::option::Option<&mut isize> = crate::unsync::OnceCell::get_mut(oncecell_6_ref_0);
    let mut result_0: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_5_ref_0, isize_4);
    let mut oncecell_7: crate::unsync::OnceCell<u128> = crate::unsync::OnceCell::clone(oncecell_4_ref_0);
    let mut result_1: std::result::Result<(), isize> = crate::sync::OnceCell::set(oncecell_3_ref_0, isize_2);
    let mut option_1: std::option::Option<isize> = crate::unsync::OnceCell::take(oncecell_2_ref_0);
    let mut option_2: std::option::Option<u128> = crate::unsync::OnceCell::into_inner(oncecell_7);
    let mut lazy_0: crate::sync::Lazy<isize, isize> = crate::sync::Lazy::new(isize_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_1);
    let mut isize_6: isize = std::option::Option::unwrap(option_1);
    crate::sync::OnceCell::clone_from(oncecell_1_ref_0, oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut isize_0: isize = 34680isize;
    let mut oncecell_0: crate::imp::OnceCell<isize> = crate::imp::OnceCell::with_value(isize_0);
    let mut oncecell_0_ref_0: &mut crate::imp::OnceCell<isize> = &mut oncecell_0;
    let mut oncecell_1: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_1_ref_0: &mut crate::imp::OnceCell<isize> = &mut oncecell_1;
    let mut oncecell_2: crate::sync::OnceCell<i128> = crate::sync::OnceCell::new();
    let mut oncecell_2_ref_0: &crate::sync::OnceCell<i128> = &mut oncecell_2;
    let mut oncecell_3: crate::sync::OnceCell<i128> = crate::sync::OnceCell::new();
    let mut oncecell_3_ref_0: &mut crate::sync::OnceCell<i128> = &mut oncecell_3;
    let mut oncecell_4: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_4_ref_0: &crate::imp::OnceCell<isize> = &mut oncecell_4;
    let mut isize_1: isize = 8579isize;
    let mut oncecell_5: crate::imp::OnceCell<std::result::Result<(), ()>> = crate::imp::OnceCell::new();
    let mut oncecell_5_ref_0: &crate::imp::OnceCell<std::result::Result<(), ()>> = &mut oncecell_5;
    let mut isize_2: isize = 10899isize;
    let mut oncecell_6: crate::imp::OnceCell<isize> = crate::imp::OnceCell::new();
    let mut oncecell_6_ref_0: &mut crate::imp::OnceCell<isize> = &mut oncecell_6;
    let mut isize_3: isize = 10503isize;
    let mut oncecell_7: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_3);
    let mut oncecell_7_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_7;
    let mut isize_4: isize = -4993isize;
    let mut oncecell_8: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::from(isize_4);
    let mut oncecell_8_ref_0: &crate::unsync::OnceCell<isize> = &mut oncecell_8;
    let mut bool_0: bool = crate::unsync::OnceCell::eq(oncecell_8_ref_0, oncecell_7_ref_0);
    let mut option_0: std::option::Option<&mut isize> = crate::imp::OnceCell::get_mut(oncecell_6_ref_0);
    let mut oncecell_9: crate::sync::OnceCell<isize> = crate::sync::OnceCell::new();
    let mut oncecell_10: crate::sync::OnceCell<isize> = crate::sync::OnceCell::with_value(isize_2);
    let mut oncecell_11: crate::unsync::OnceCell<isize> = crate::unsync::OnceCell::with_value(isize_1);
    crate::imp::OnceCell::wait(oncecell_4_ref_0);
    crate::sync::OnceCell::clone_from(oncecell_3_ref_0, oncecell_2_ref_0);
    let mut option_1: std::option::Option<&mut isize> = crate::imp::OnceCell::get_mut(oncecell_1_ref_0);
    let mut option_2: std::option::Option<&mut isize> = crate::imp::OnceCell::get_mut(oncecell_0_ref_0);
    panic!("From RustyUnit with love");
}
}