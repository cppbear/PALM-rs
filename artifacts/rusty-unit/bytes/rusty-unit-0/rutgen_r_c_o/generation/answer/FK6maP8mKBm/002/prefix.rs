// Answer 0

#[test]
fn test_shared_to_vec_impl_non_unique() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(2),
    }));
    let ptr = Box::into_raw(Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])) as *const u8;
    let len = 10;

    unsafe {
        let result = shared_to_vec_impl(shared, ptr, len);
    }
}

#[test]
fn test_shared_to_vec_impl_zero_length() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(2),
    }));
    let ptr = Box::into_raw(Box::new([])) as *const u8;
    let len = 0;

    unsafe {
        let result = shared_to_vec_impl(shared, ptr, len);
    }
}

#[test]
fn test_shared_to_vec_impl_large_ref_count() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(10),
    }));
    let ptr = Box::into_raw(Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])) as *const u8;
    let len = 10;

    unsafe {
        let result = shared_to_vec_impl(shared, ptr, len);
    }
}

#[test]
fn test_shared_to_vec_impl_high_ref_count() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(3),
    }));
    let ptr = Box::into_raw(Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])) as *const u8;
    let len = 10;

    unsafe {
        let result = shared_to_vec_impl(shared, ptr, len);
    }
}

