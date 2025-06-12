// Answer 0

#[test]
fn test_as_slice_valid() {
    struct BytesMut {
        ptr: std::ptr::NonNull<u8>,
        len: usize,
    }

    fn create_bytes_mut(len: usize) -> BytesMut {
        let vec = vec![1u8; len];
        let ptr = std::ptr::NonNull::new(vec.as_ptr() as *mut u8).unwrap();
        std::mem::forget(vec); // Prevent Vec from dropping and deallocating
        BytesMut { ptr, len }
    }

    let bytes_mut = create_bytes_mut(5);
    let slice = unsafe { std::slice::from_raw_parts(bytes_mut.ptr.as_ptr(), bytes_mut.len) };
    assert_eq!(slice, &[1, 1, 1, 1, 1]);
}

#[test]
fn test_as_slice_zero_length() {
    struct BytesMut {
        ptr: std::ptr::NonNull<u8>,
        len: usize,
    }

    fn create_bytes_mut(len: usize) -> BytesMut {
        let vec = vec![0u8; len];
        let ptr = std::ptr::NonNull::new(vec.as_ptr() as *mut u8).unwrap();
        std::mem::forget(vec); // Prevent Vec from dropping and deallocating
        BytesMut { ptr, len }
    }

    let bytes_mut = create_bytes_mut(0);
    let slice = unsafe { std::slice::from_raw_parts(bytes_mut.ptr.as_ptr(), bytes_mut.len) };
    assert!(slice.is_empty());
}

