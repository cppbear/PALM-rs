// Answer 0

#[test]
fn test_shared_to_mut_valid_single_ref() {
    let buf = Box::into_raw(Box::new([1u8, 2, 3, 4]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 4,
        ref_cnt: AtomicUsize::new(1),
    }));
    let atomic_ptr = AtomicPtr::new(shared);
    let ptr = buf as *const u8;
    let len = 4;

    unsafe {
        let _result = shared_to_mut(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_shared_to_mut_multiple_ref() {
    let buf = Box::into_raw(Box::new([5u8, 6, 7, 8]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 4,
        ref_cnt: AtomicUsize::new(2),
    }));
    let atomic_ptr = AtomicPtr::new(shared);
    let ptr = buf as *const u8;
    let len = 4;

    unsafe {
        let _result = shared_to_mut(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_shared_to_mut_edge_case_zero_length() {
    let buf = Box::into_raw(Box::new([]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    }));
    let atomic_ptr = AtomicPtr::new(shared);
    let ptr = buf as *const u8;
    let len = 0;

    unsafe {
        let _result = shared_to_mut(&atomic_ptr, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_shared_to_mut_invalid_length() {
    let buf = Box::into_raw(Box::new([1u8, 2]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 2,
        ref_cnt: AtomicUsize::new(1),
    }));
    let atomic_ptr = AtomicPtr::new(shared);
    let ptr = buf as *const u8;
    let len = usize::MAX;  // Exceeding length

    unsafe {
        let _result = shared_to_mut(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_shared_to_mut_small_valid_range() {
    let buf = Box::into_raw(Box::new([9u8, 10]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 2,
        ref_cnt: AtomicUsize::new(1),
    }));
    let atomic_ptr = AtomicPtr::new(shared);
    let ptr = buf as *const u8;
    let len = 2;

    unsafe {
        let _result = shared_to_mut(&atomic_ptr, ptr, len);
    }
}

