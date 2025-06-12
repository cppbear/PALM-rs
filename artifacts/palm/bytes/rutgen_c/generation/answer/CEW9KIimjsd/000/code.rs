// Answer 0

#[test]
fn test_shared_v_to_vec_unique() {
    use core::sync::atomic::AtomicPtr;
    use alloc::vec::Vec;
    use alloc::boxed::Box;

    struct TemporaryShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let data_ptr = AtomicPtr::new(Box::into_raw(Box::new(TemporaryShared {
        vec: Vec::from(&b"hello"[..]),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    })) as *mut ());

    let source_slice = b"world";
    let len = source_slice.len();

    unsafe {
        let result = shared_v_to_vec(&data_ptr, source_slice.as_ptr(), len);
        assert_eq!(result, b"world");
        assert_eq!(data_ptr.load(core::sync::atomic::Ordering::Relaxed), core::ptr::null_mut());
    }
}

#[test]
fn test_shared_v_to_vec_non_unique() {
    use core::sync::atomic::AtomicPtr;
    use alloc::vec::Vec;
    use alloc::boxed::Box;

    struct TemporaryShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let shared_instance = Box::new(TemporaryShared {
        vec: Vec::from(&b"hello"[..]),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),  // Set ref_count to 2 to make it non-unique
    });
    
    let data_ptr = AtomicPtr::new(Box::into_raw(shared_instance) as *mut ());

    let source_slice = b"world";
    let len = source_slice.len();

    unsafe {
        let result = shared_v_to_vec(&data_ptr, source_slice.as_ptr(), len);
        assert_eq!(result, b"world");
        assert_eq!(data_ptr.load(core::sync::atomic::Ordering::Relaxed), core::ptr::null_mut());
    }
}

