// Answer 0

#[test]
fn test_shared_v_to_mut_unique() {
    use core::sync::atomic::AtomicPtr;
    use alloc::vec;

    let vec_data = vec![1, 2, 3, 4];
    let mut shared = Shared {
        vec: vec_data.clone(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    
    let data = AtomicPtr::new(&mut shared as *mut Shared as *mut ());

    unsafe {
        let result = shared_v_to_mut(&data, vec_data.as_ptr(), vec_data.len());
        assert_eq!(result.len(), vec_data.len());
        assert_eq!(result.capacity(), vec_data.len());
    }
}

#[test]
fn test_shared_v_to_mut_not_unique() {
    use core::sync::atomic::AtomicPtr;
    use alloc::vec;

    let vec_data = vec![1, 2, 3, 4];
    let mut shared = Shared {
        vec: vec_data.clone(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    };
    
    let data = AtomicPtr::new(&mut shared as *mut Shared as *mut ());

    unsafe {
        let result = shared_v_to_mut(&data, vec_data.as_ptr(), vec_data.len());
        assert_eq!(result.len(), vec_data.len());
        assert_eq!(result.capacity(), vec_data.len());
    }
}

