// Answer 0

#[test]
fn test_borrow_empty_bytes_mut() {
    let empty_bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 0]))).unwrap(),
        len: 0,
        cap: 0,
        data: &SHARED_VTABLE as *const Vtable as *mut Shared,
    };
    let _ = empty_bytes_mut.borrow();
}

#[test]
fn test_borrow_small_bytes_mut() {
    let small_bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1u8, 2, 3]))).unwrap(),
        len: 3,
        cap: 3,
        data: &SHARED_VTABLE as *const Vtable as *mut Shared,
    };
    let _ = small_bytes_mut.borrow();
}

#[test]
fn test_borrow_large_bytes_mut() {
    let large_bytes_mut: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 1024]))).unwrap(),
        len: 1024,
        cap: 1024,
        data: &SHARED_VTABLE as *const Vtable as *mut Shared,
    };
    let _ = large_bytes_mut.borrow();
}

#[test]
#[should_panic]
fn test_borrow_null_pointer() {
    let null_bytes_mut = BytesMut {
        ptr: NonNull::new(ptr::null_mut()).unwrap(),
        len: 0,
        cap: 0,
        data: &SHARED_VTABLE as *const Vtable as *mut Shared,
    };
    let _ = null_bytes_mut.borrow();
}

