// Answer 0

#[test]
fn test_shared_is_unique_with_ref_count_one() {
    let shared = Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    };
    let data = AtomicPtr::new(&shared as *const _ as *mut _);
    let result = unsafe { shared_is_unique(&data) };
}

#[test]
fn test_shared_is_unique_with_ref_count_two() {
    let shared = Shared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(2),
    };
    let data = AtomicPtr::new(&shared as *const _ as *mut _);
    let result = unsafe { shared_is_unique(&data) };
}

#[test]
#[should_panic]
fn test_shared_is_unique_with_null_ptr() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let result = unsafe { shared_is_unique(&data) };
}

