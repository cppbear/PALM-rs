// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    let mut vec = Vec::with_capacity(5);
    vec.extend_from_slice(&[1, 2, 3, 4, 5]);
    
    let test_bytes_mut = TestBytesMut {
        ptr: NonNull::new(vec.as_mut_ptr()).unwrap(),
        len: 5,
        cap: 5,
        data: std::ptr::null_mut(),
    };

    let slice = unsafe { test_bytes_mut.as_slice() };
    assert_eq!(slice, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_as_slice_empty() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    let test_bytes_mut = TestBytesMut {
        ptr: NonNull::new(std::ptr::null_mut()).unwrap(),
        len: 0,
        cap: 0,
        data: std::ptr::null_mut(),
    };

    let slice = unsafe { test_bytes_mut.as_slice() };
    assert_eq!(slice.len(), 0);
}

#[test]
#[should_panic]
fn test_as_slice_invalid_length() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    let mut vec = Vec::with_capacity(5);
    vec.extend_from_slice(&[1, 2, 3, 4, 5]);

    let test_bytes_mut = TestBytesMut {
        ptr: NonNull::new(vec.as_mut_ptr()).unwrap(),
        len: 10, // Intentional invalid length
        cap: 5,
        data: std::ptr::null_mut(),
    };

    let _slice = unsafe { test_bytes_mut.as_slice() }; // This should panic
}

