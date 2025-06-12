// Answer 0

#[test]
fn test_deref_empty_bytes_mut() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 0]))).unwrap(),
        len: 0,
        cap: 0,
        data: &mut Shared {
            vec: Vec::new(),
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(0),
        } as *mut Shared,
    };
    let result: &[u8] = &bytes_mut;
    assert_eq!(result.len(), 0);
}

#[test]
fn test_deref_non_empty_bytes_mut() {
    let data = vec![1u8, 2, 3];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(data.clone().into_boxed_slice())).unwrap(),
        len: data.len(),
        cap: data.capacity(),
        data: &mut Shared {
            vec: data,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        } as *mut Shared,
    };
    let result: &[u8] = &bytes_mut;
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_deref_with_capacity() {
    let data = vec![4u8, 5, 6, 7, 8];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(data.clone().into_boxed_slice())).unwrap(),
        len: data.len(),
        cap: data.capacity(),
        data: &mut Shared {
            vec: data,
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        } as *mut Shared,
    };
    let result: &[u8] = &bytes_mut;
    assert_eq!(result.len(), 5);
    assert_eq!(result, &[4, 5, 6, 7, 8]);
}

