// Answer 0

#[test]
fn test_release_shared_ref_count_not_one() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    unsafe fn release_shared(ptr: *mut Shared) {
        if (*ptr).ref_cnt.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        (*ptr).ref_cnt.load(Ordering::Acquire);
        drop(Box::from_raw(ptr));
    }

    // Create a Shared instance with a reference count of 2
    let shared = Shared {
        ref_cnt: AtomicUsize::new(2),
    };

    // Create a mutable pointer to the Shared instance
    let shared_ptr = Box::into_raw(Box::new(shared));

    // Call release_shared and expect no panic
    unsafe {
        release_shared(shared_ptr);
    }

    // Ensure ref count is now 1 as we decremented it
    let ref_count = unsafe { (*shared_ptr).ref_cnt.load(Ordering::SeqCst) };
    assert_eq!(ref_count, 1);

    // Clean up to avoid memory leaks
    unsafe {
        release_shared(shared_ptr);
    }
}

#[test]
#[should_panic]
fn test_release_shared_ref_count_zero() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    unsafe fn release_shared(ptr: *mut Shared) {
        if (*ptr).ref_cnt.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        (*ptr).ref_cnt.load(Ordering::Acquire);
        drop(Box::from_raw(ptr));
    }

    // Create a Shared instance with a reference count of 1
    let shared = Shared {
        ref_cnt: AtomicUsize::new(1),
    };

    // Create a mutable pointer to the Shared instance
    let shared_ptr = Box::into_raw(Box::new(shared));

    // Call release_shared to decrement ref count to 0
    unsafe {
        release_shared(shared_ptr);
    }

    // The next call should panic since the ref count is now 0
    unsafe {
        release_shared(shared_ptr);
    }
}

