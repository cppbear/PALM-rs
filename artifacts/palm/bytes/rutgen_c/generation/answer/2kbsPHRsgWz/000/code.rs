// Answer 0

#[test]
fn test_promotable_is_unique_arc_unique() {
    let shared_buf = Box::into_raw(Box::new(Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    }));
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut ());

    unsafe {
        let result = promotable_is_unique(&atomic_ptr);
        assert!(result);
    }

    unsafe {
        // Clean up
        let _ = Box::from_raw(shared_buf);
    }
}

#[test]
fn test_promotable_is_unique_arc_not_unique() {
    let shared_buf = Box::into_raw(Box::new(Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(2),
    }));
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut ());

    unsafe {
        let result = promotable_is_unique(&atomic_ptr);
        assert!(!result);
    }

    unsafe {
        // Clean up
        let _ = Box::from_raw(shared_buf);
    }
}

#[test]
fn test_promotable_is_unique_non_arc() {
    let atomic_ptr = AtomicPtr::new(0 as *mut ());

    unsafe {
        let result = promotable_is_unique(&atomic_ptr);
        assert!(result);
    }
}

