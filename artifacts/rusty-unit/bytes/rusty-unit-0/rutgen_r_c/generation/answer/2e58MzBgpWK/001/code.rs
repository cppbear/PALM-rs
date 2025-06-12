// Answer 0

#[test]
fn test_promotable_even_clone_arc() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};

    struct TestShared {
        ref_cnt: AtomicUsize,
    }

    let shared = Box::new(TestShared {
        ref_cnt: AtomicUsize::new(1),
    });

    let shared_ptr = Box::into_raw(shared);
    let atomic_ptr = AtomicPtr::new(shared_ptr as *mut () | KIND_ARC as usize);
    let data_ptr: *const u8 = null_mut(); // Pointing to no valid data
    let len: usize = 0; // Length of 0 for the test

    unsafe {
        let result = promotable_even_clone(&atomic_ptr, data_ptr, len);
        // Validate the result
        assert_eq!(result.len, len);
        assert_eq!(result.ptr, data_ptr);
        // Validate the reference count has increased
        assert_eq!((*shared_ptr).ref_cnt.load(Ordering::Relaxed), 2);
    }

    // Clean up the shared pointer
    unsafe {
        let _ = atomic_ptr.load(Ordering::Relaxed) as *mut TestShared;
    }
}

