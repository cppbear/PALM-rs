// Answer 0

#[test]
fn test_deref_empty_bytes_mut() {
    let vec = Vec::new();
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let bytes_mut = BytesMut {
        ptr,
        len: 0,
        cap: vec.capacity(),
        data: Box::into_raw(Box::new(Shared {
            vec,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })),
    };
    let _ = bytes_mut.deref();
}

#[test]
fn test_deref_zero_length_bytes_mut() {
    let vec = Vec::with_capacity(10);
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let bytes_mut = BytesMut {
        ptr,
        len: 0,
        cap: vec.capacity(),
        data: Box::into_raw(Box::new(Shared {
            vec,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })),
    };
    let _ = bytes_mut.deref();
}

#[test]
fn test_deref_non_empty_bytes_mut() {
    let vec = vec![1, 2, 3, 4, 5];
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let bytes_mut = BytesMut {
        ptr,
        len: vec.len(),
        cap: vec.capacity(),
        data: Box::into_raw(Box::new(Shared {
            vec,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })),
    };
    let _ = bytes_mut.deref();
}

#[test]
fn test_deref_large_bytes_mut() {
    let vec = vec![0u8; usize::MAX as usize];
    let ptr = NonNull::new(vec.as_ptr() as *mut u8).unwrap();
    let bytes_mut = BytesMut {
        ptr,
        len: vec.len(),
        cap: vec.capacity(),
        data: Box::into_raw(Box::new(Shared {
            vec,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })),
    };
    let _ = bytes_mut.deref();
}

#[test]
#[should_panic]
fn test_deref_invalid_pointer() {
    let invalid_ptr: NonNull<u8> = NonNull::new(0x1 as *mut u8).unwrap();
    let bytes_mut = BytesMut {
        ptr: invalid_ptr,
        len: 10,
        cap: 10,
        data: Box::into_raw(Box::new(Shared {
            vec: Vec::new(),
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })),
    };
    let _ = bytes_mut.deref();
}

