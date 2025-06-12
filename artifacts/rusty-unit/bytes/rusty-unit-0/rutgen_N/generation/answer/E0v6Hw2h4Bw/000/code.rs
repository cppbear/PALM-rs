// Answer 0

#[test]
fn test_as_slice_mut_valid() {
    struct BytesMut {
        ptr: std::ptr::NonNull<u8>,
        len: usize,
    }

    let mut data = vec![1u8, 2, 3, 4];
    let ptr = std::ptr::NonNull::new(data.as_mut_ptr()).unwrap();
    let mut bytes_mut = BytesMut { ptr, len: data.len() };
    
    let slice: &mut [u8] = bytes_mut.as_slice_mut();
    slice[0] = 10;
    
    assert_eq!(slice, &[10, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_as_slice_mut_invalid_length() {
    struct BytesMut {
        ptr: std::ptr::NonNull<u8>,
        len: usize,
    }

    let mut data = vec![1u8, 2, 3, 4];
    let ptr = std::ptr::NonNull::new(data.as_mut_ptr()).unwrap();
    let mut bytes_mut = BytesMut { ptr, len: 2 };
    
    let _slice: &mut [u8] = bytes_mut.as_slice_mut();
}

