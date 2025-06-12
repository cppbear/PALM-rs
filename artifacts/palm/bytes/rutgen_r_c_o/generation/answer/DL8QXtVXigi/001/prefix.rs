// Answer 0

#[test]
fn test_promotable_to_vec_arc_zero_length() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(Shared { buf: ptr::null_mut(), cap: 0, ref_cnt: AtomicUsize::new(1) })));
    let ptr: *const u8 = ptr::null();
    let len = 0;
    let f: fn(*mut ()) -> *mut u8 = |shared| shared as *mut u8;

    unsafe {
        promotable_to_vec(&data, ptr, len, f);
    }
}

#[test]
fn test_promotable_to_vec_arc_non_zero_length() {
    let mut buffer = vec![1u8, 2, 3, 4];
    let data = AtomicPtr::new(Box::into_raw(Box::new(Shared { buf: buffer.as_mut_ptr(), cap: buffer.len(), ref_cnt: AtomicUsize::new(1) })));
    let ptr: *const u8 = buffer.as_ptr();
    let len = buffer.len();
    let f: fn(*mut ()) -> *mut u8 = |shared| shared as *mut u8;

    unsafe {
        promotable_to_vec(&data, ptr, len, f);
    }
}

#[test]
fn test_promotable_to_vec_arc_large_length() {
    let mut buffer = vec![0u8; 1024]; // allocate a large buffer
    let data = AtomicPtr::new(Box::into_raw(Box::new(Shared { buf: buffer.as_mut_ptr(), cap: buffer.len(), ref_cnt: AtomicUsize::new(1) })));
    let ptr: *const u8 = buffer.as_ptr();
    let len = buffer.len();
    let f: fn(*mut ()) -> *mut u8 = |shared| shared as *mut u8;

    unsafe {
        promotable_to_vec(&data, ptr, len, f);
    }
}

#[test]
#[should_panic]
fn test_promotable_to_vec_arc_invalid_reference_count() {
    let not_unique_shared = Box::into_raw(Box::new(Shared { buf: ptr::null_mut(), cap: 0, ref_cnt: AtomicUsize::new(2) }));
    let data = AtomicPtr::new(not_unique_shared);
    let ptr: *const u8 = ptr::null();
    let len = 1;
    let f: fn(*mut ()) -> *mut u8 = |shared| shared as *mut u8;

    unsafe {
        promotable_to_vec(&data, ptr, len, f);
    }
}

