// Answer 0

#[test]
fn test_try_unsplit_other_capacity_zero() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.resize(5, 1); // Initialize with some data

    let other = BytesMut::new(); // other has capacity of 0

    let result = bytes_mut.try_unsplit(other);
    assert!(result.is_ok());
}

#[test]
fn test_try_unsplit_ptr_not_equal() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let other = BytesMut::with_capacity(10);
    let result = bytes_mut.try_unsplit(other);
    assert!(result.is_err());
}

#[test]
fn test_try_unsplit_contiguous_failed() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 1);

    let other = BytesMut::with_capacity(5);
    unsafe {
        bytes_mut.set_len(5);
        other.set_len(5); // Ensure lengths are equal
    }

    let result = bytes_mut.try_unsplit(other);
    assert!(result.is_err());
}

