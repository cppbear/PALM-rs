// Answer 0

#[test]
fn test_shared_v_is_unique_unique() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
    use alloc::vec::Vec;
    
    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let data = AtomicPtr::new(Box::into_raw(shared));

    unsafe {
        assert!(shared_v_is_unique(&data));
    }

    // Clean up the allocated memory
    let ptr = data.load(Ordering::Acquire);
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr);
    }
}

#[test]
fn test_shared_v_is_unique_not_unique() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
    use alloc::vec::Vec;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    });

    let data = AtomicPtr::new(Box::into_raw(shared));

    unsafe {
        assert!(!shared_v_is_unique(&data));
    }

    // Clean up the allocated memory
    let ptr = data.load(Ordering::Acquire);
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr);
    }
}

