// Answer 0

#[test]
fn test_increment_shared_not_panic() {
    use core::ptr::NonNull;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use alloc::vec::Vec;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    unsafe fn increment_shared(ptr: *mut Shared) {
        let old_size = (*ptr).ref_count.fetch_add(1, Ordering::Relaxed);
        if old_size > isize::MAX as usize {
            crate::abort();
        }
    }

    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };

    let ptr = NonNull::from(&shared as *const Shared as *mut Shared);
    unsafe {
        increment_shared(ptr.as_ptr());
        assert_eq!(shared.ref_count.load(Ordering::Relaxed), isize::MAX as usize + 1);
    }
}

#[should_panic]
#[test]
fn test_increment_shared_panic() {
    use core::ptr::NonNull;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use alloc::vec::Vec;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    unsafe fn increment_shared(ptr: *mut Shared) {
        let old_size = (*ptr).ref_count.fetch_add(1, Ordering::Relaxed);
        if old_size > isize::MAX as usize {
            crate::abort();
        }
    }

    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };

    let ptr = NonNull::from(&shared as *const Shared as *mut Shared);
    unsafe {
        // Increment the ref_count to isize::MAX, which should trigger the panic on the next increment
        shared.ref_count.fetch_add(1, Ordering::Relaxed);
        increment_shared(ptr.as_ptr());
    }
}

