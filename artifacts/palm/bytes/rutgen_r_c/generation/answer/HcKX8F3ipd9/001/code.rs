// Answer 0

#[test]
fn test_as_slice_empty() {
    let mut bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.as_slice(), &[]);
}

#[test]
fn test_as_slice_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(5);
        ptr::copy_nonoverlapping([1, 2, 3, 4, 5].as_ptr(), bytes_mut.ptr.as_ptr(), 5);
    }
    assert_eq!(bytes_mut.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
#[should_panic]
fn test_as_slice_invalid_length() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.set_len(1);
        // Intentionally causing an invalid dereference by not initializing the memory
        let _ = bytes_mut.as_slice();
    }
}

#[test]
fn test_as_slice_full_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(10);
        ptr::copy_nonoverlapping([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_ptr(), bytes_mut.ptr.as_ptr(), 10);
    }
    assert_eq!(bytes_mut.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

