// Answer 0

#[test]
fn test_owned_clone_valid() {
    use core::sync::atomic::AtomicPtr;
    use core::ptr::null_mut;

    struct MockOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    let mut mock_instance = MockOwnedLifetime {
        ref_cnt: AtomicUsize::new(1),
        drop: std::mem::forget,
    };

    let atomic_ptr = AtomicPtr::new(&mut mock_instance as *mut _ as *mut ());
    let original_ptr: *const u8 = &4u8;

    unsafe {
        let cloned_bytes = owned_clone(&atomic_ptr, original_ptr, 1);
        assert_eq!(cloned_bytes.len, 1);
        assert_eq!(cloned_bytes.ptr, original_ptr);
        
        let new_ref_cnt = mock_instance.ref_cnt.load(core::sync::atomic::Ordering::Relaxed);
        assert_eq!(new_ref_cnt, 2);
    }
}

#[test]
#[should_panic]
fn test_owned_clone_exceed_ref_count() {
    use core::sync::atomic::AtomicPtr;
    use core::ptr::null_mut;

    struct MockOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    let mut mock_instance = MockOwnedLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1), // Set to maximum allowed
        drop: std::mem::forget,
    };

    let atomic_ptr = AtomicPtr::new(&mut mock_instance as *mut _ as *mut ());
    let original_ptr: *const u8 = &4u8;

    unsafe {
        owning_clone(&atomic_ptr, original_ptr, 1); // This should exceed max and panic
    }
}

