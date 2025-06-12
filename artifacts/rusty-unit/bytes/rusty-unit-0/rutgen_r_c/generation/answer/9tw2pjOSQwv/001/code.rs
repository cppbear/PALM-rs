// Answer 0

#[test]
fn test_release_shared_non_zero_ref_count() {
    use core::ptr::null_mut;
    use core::sync::atomic::AtomicUsize;
    use alloc::boxed::Box;

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    // Create a Shared instance with a non-zero reference count
    let shared = Box::new(Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(2), // Set reference count to 2
    });

    // Convert the Box to a raw pointer
    let ptr = Box::into_raw(shared);

    unsafe {
        // Call release_shared function
        release_shared(ptr); // This should decrease ref_cnt to 1 and not drop the shared instance

        // Verify that the reference count is still greater than zero
        assert_eq!((*ptr).ref_cnt.load(core::sync::atomic::Ordering::Relaxed), 1);

        // Cleanup: properly release the reference by calling release_shared again
        release_shared(ptr); // This should decrement ref_cnt to 0 and drop the shared instance
    }
}

#[test]
#[should_panic]
fn test_release_shared_zero_ref_count() {
    use core::ptr::null_mut;
    use core::sync::atomic::AtomicUsize;
    use alloc::boxed::Box;

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    // Create a Shared instance with a reference count of 1
    let shared = Box::new(Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1), // Set reference count to 1 for triggering panic
    });

    // Convert the Box to a raw pointer
    let ptr = Box::into_raw(shared);

    unsafe {
        // Call release_shared function
        release_shared(ptr); // This should decrease ref_cnt to 0 and drop the shared instance

        // Attempt to access it after it has been dropped (should panic as it's dangling)
        let _ = (*ptr).ref_cnt.load(core::sync::atomic::Ordering::Relaxed); // Dangling reference access
    }
}

