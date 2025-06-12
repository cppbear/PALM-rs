// Answer 0

#[test]
fn test_increment_shared_at_max_size() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_count: AtomicUsize,
    }

    unsafe fn increment_shared(ptr: *mut Shared) {
        let old_size = (*ptr).ref_count.fetch_add(1, Ordering::Relaxed);

        if old_size > isize::MAX as usize {
            crate::abort();
        }
    }

    // Initialize Shared with max size
    let shared = Shared {
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };

    let shared_ptr: *mut Shared = &shared as *const _ as *mut _;

    // Call the function, should not panic
    unsafe {
        increment_shared(shared_ptr);
    }

    // Verify the ref_count has incremented to isize::MAX as usize + 1
    assert_eq!(shared.ref_count.load(Ordering::Relaxed), isize::MAX as usize + 1);
}

#[test]
#[should_panic]
fn test_increment_shared_above_max_size() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_count: AtomicUsize,
    }

    unsafe fn increment_shared(ptr: *mut Shared) {
        let old_size = (*ptr).ref_count.fetch_add(1, Ordering::Relaxed);

        if old_size > isize::MAX as usize {
            crate::abort();
        }
    }

    // Initialize Shared at max size, will panic on next increment
    let shared = Shared {
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };

    let shared_ptr: *mut Shared = &shared as *const _ as *mut _;

    // Calling this should trigger a panic
    unsafe {
        increment_shared(shared_ptr);
        increment_shared(shared_ptr); // This should cause a panic
    }
}

