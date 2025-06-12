// Answer 0

#[test]
fn test_shared_v_clone_zero_length() {
    let atomic_ptr = AtomicPtr::new(Box::into_raw(Box::new(Shared {
        vec: vec![0u8; 0],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    })) as *mut ());
    let ptr = std::ptr::non_null::NonNull::dangling().as_ptr();
    let len = 0;
    let _ = unsafe { shared_v_clone(&atomic_ptr, ptr, len) };
}

#[test]
fn test_shared_v_clone_non_empty_length() {
    let atomic_ptr = AtomicPtr::new(Box::into_raw(Box::new(Shared {
        vec: vec![1u8, 2u8, 3u8],
        original_capacity_repr: 1,
        ref_count: AtomicUsize::new(1),
    })) as *mut ());
    let ptr = [1, 2, 3].as_ptr();
    let len = 3;
    let _ = unsafe { shared_v_clone(&atomic_ptr, ptr, len) };
}

#[test]
fn test_shared_v_clone_max_size() {
    let atomic_ptr = AtomicPtr::new(Box::into_raw(Box::new(Shared {
        vec: vec![0u8; usize::MAX],
        original_capacity_repr: usize::MAX,
        ref_count: AtomicUsize::new(1),
    })) as *mut ());
    let ptr = vec![0u8; usize::MAX].as_ptr();
    let len = usize::MAX;
    let _ = unsafe { shared_v_clone(&atomic_ptr, ptr, len) };
}

#[test]
fn test_shared_v_clone_with_valid_pointer() {
    let data = vec![10u8, 20u8, 30u8];
    let atomic_ptr = AtomicPtr::new(Box::into_raw(Box::new(Shared {
        vec: data.clone(),
        original_capacity_repr: 3,
        ref_count: AtomicUsize::new(1),
    })) as *mut ());
    let ptr = data.as_ptr();
    let len = data.len();
    let _ = unsafe { shared_v_clone(&atomic_ptr, ptr, len) };
}

