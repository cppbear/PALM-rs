// Answer 0

#[test]
fn test_borrow_mut_empty() {
    let vec = Vec::new();
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let data = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let bytes_mut = BytesMut {
        ptr,
        len: 0,
        cap: 0,
        data: Box::into_raw(data),
    };
    let _slice: &mut [u8] = bytes_mut.borrow_mut();
}

#[test]
fn test_borrow_mut_with_data() {
    let vec = vec![1, 2, 3];
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let data = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let bytes_mut = BytesMut {
        ptr,
        len: 3,
        cap: 3,
        data: Box::into_raw(data),
    };
    let _slice: &mut [u8] = bytes_mut.borrow_mut();
}

#[test]
fn test_borrow_mut_full_capacity() {
    let vec = vec![4; MAX_VEC_POS];
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let data = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let bytes_mut = BytesMut {
        ptr,
        len: MAX_VEC_POS,
        cap: MAX_VEC_POS,
        data: Box::into_raw(data),
    };
    let _slice: &mut [u8] = bytes_mut.borrow_mut();
}

#[test]
#[should_panic]
fn test_borrow_mut_invalid_pointer() {
    let vec = vec![];
    let ptr = NonNull::new(0 as *mut u8).unwrap(); // invalid pointer
    let data = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let bytes_mut = BytesMut {
        ptr,
        len: 0,
        cap: 0,
        data: Box::into_raw(data),
    };
    let _slice: &mut [u8] = bytes_mut.borrow_mut(); // should panic
}

#[test]
fn test_borrow_mut_partial_capacity() {
    let vec = vec![5, 6, 7];
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let data = Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });
    let bytes_mut = BytesMut {
        ptr,
        len: 2,
        cap: 3,
        data: Box::into_raw(data),
    };
    let _slice: &mut [u8] = bytes_mut.borrow_mut();
}

