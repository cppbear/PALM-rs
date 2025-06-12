// Answer 0

#[test]
fn test_shared_clone_valid_input() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 16])) as *mut u8,
        cap: 16,
        ref_cnt: AtomicUsize::new(1),
    }));
    let data = AtomicPtr::new(shared as _);
    let ptr: *const u8 = unsafe { (*shared).buf };
    let len: usize = 10;

    unsafe {
        let _result = shared_clone(&data, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_shared_clone_null_pointer() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 16])) as *mut u8,
        cap: 16,
        ref_cnt: AtomicUsize::new(1),
    }));
    let data = AtomicPtr::new(shared as _);
    let ptr: *const u8 = core::ptr::null();
    let len: usize = 10;

    unsafe {
        let _result = shared_clone(&data, ptr, len);
    }
}

#[test]
fn test_shared_clone_edge_length() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 16])) as *mut u8,
        cap: 16,
        ref_cnt: AtomicUsize::new(1),
    }));
    let data = AtomicPtr::new(shared as _);
    let ptr: *const u8 = unsafe { (*shared).buf };
    let len: usize = usize::MAX / 2;

    unsafe {
        let _result = shared_clone(&data, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_shared_clone_exceed_max_length() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 16])) as *mut u8,
        cap: 16,
        ref_cnt: AtomicUsize::new(1),
    }));
    let data = AtomicPtr::new(shared as _);
    let ptr: *const u8 = unsafe { (*shared).buf };
    let len: usize = usize::MAX;

    unsafe {
        let _result = shared_clone(&data, ptr, len);
    }
}

