// Answer 0

#[test]
fn test_shared_v_to_mut_unique_case_zero_length() {
    let vec = vec![1, 2, 3, 4];
    let mut shared = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    let ptr = NonNull::new(vec.as_mut_ptr()).unwrap().as_ptr();
    let len = 0;

    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_to_mut_unique_case_full_length() {
    let vec = vec![5, 6, 7, 8];
    let mut shared = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    let ptr = NonNull::new(vec.as_mut_ptr()).unwrap().as_ptr();
    let len = vec.len();

    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_to_mut_unique_case_some_length() {
    let vec = vec![9, 10, 11, 12, 13];
    let mut shared = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    let ptr = NonNull::new(vec.as_mut_ptr()).unwrap().as_ptr();
    let len = 3;

    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_to_mut_unique_case_max_length() {
    let vec = vec![14; MAX_VEC_POS]; // using a valid length close to the max
    let mut shared = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    let ptr = NonNull::new(vec.as_mut_ptr()).unwrap().as_ptr();
    let len = MAX_VEC_POS;

    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

