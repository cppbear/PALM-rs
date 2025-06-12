// Answer 0

#[test]
fn test_deref_mut_non_empty_buffer() {
    let vec = vec![1, 2, 3, 4, 5];
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(vec.as_mut_ptr()).unwrap(),
        len: vec.len(),
        cap: vec.capacity(),
        data: Box::into_raw(Box::new(Shared {
            vec,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })),
    };

    let slice: &mut [u8] = bytes_mut.deref_mut();
    assert_eq!(slice, [1, 2, 3, 4, 5]);
}

#[test]
fn test_deref_mut_empty_buffer() {
    let vec: Vec<u8> = Vec::new();
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(vec.as_mut_ptr()).unwrap(),
        len: vec.len(),
        cap: vec.capacity(),
        data: Box::into_raw(Box::new(Shared {
            vec,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })),
    };

    let slice: &mut [u8] = bytes_mut.deref_mut();
    assert_eq!(slice.len(), 0);
}

#[should_panic]
fn test_deref_mut_invalid_pointer() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(0 as *mut u8).unwrap(),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };

    let _slice: &mut [u8] = bytes_mut.deref_mut();
}

