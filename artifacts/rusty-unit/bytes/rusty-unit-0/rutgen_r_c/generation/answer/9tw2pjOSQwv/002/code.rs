// Answer 0

#[test]
fn test_release_shared_reference_count_not_one() {
    use core::ptr::null_mut;
    use alloc::alloc::{alloc, dealloc, Layout};

    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe fn create_shared(capacity: usize) -> *mut TestShared {
        let layout = Layout::from_size_align(std::mem::size_of::<TestShared>(), std::mem::align_of::<TestShared>()).unwrap();
        let ptr = alloc(layout) as *mut TestShared;

        (*ptr) = TestShared {
            buf: null_mut(), // No allocation for the buffer since we are testing reference count behavior
            cap: capacity,
            ref_cnt: AtomicUsize::new(2), // Initialize reference count to 2 to satisfy the test condition
        };

        ptr
    }

    unsafe {
        let shared_ptr = create_shared(0);

        // Call the function, it should not drop the shared object as ref_cnt is 2
        release_shared(shared_ptr);

        // Verify that the reference count was decremented correctly
        assert_eq!((*shared_ptr).ref_cnt.load(Ordering::Relaxed), 1);

        // Cleanup: Call release_shared again to clean up
        release_shared(shared_ptr);
    }
}

#[test]
#[should_panic]
fn test_release_shared_reference_count_exactly_one() {
    use core::ptr::null_mut;
    use alloc::alloc::{alloc, dealloc, Layout};

    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe fn create_shared(capacity: usize) -> *mut TestShared {
        let layout = Layout::from_size_align(std::mem::size_of::<TestShared>(), std::mem::align_of::<TestShared>()).unwrap();
        let ptr = alloc(layout) as *mut TestShared;

        (*ptr) = TestShared {
            buf: null_mut(), // No allocation for the buffer since we are testing reference count behavior
            cap: capacity,
            ref_cnt: AtomicUsize::new(1), // Initialize reference count to 1 to trigger panic
        };

        ptr
    }

    unsafe {
        let shared_ptr = create_shared(0);

        // Call the function, this should cause a panic on the next release since ref_cnt is 1
        release_shared(shared_ptr);
    }
}

