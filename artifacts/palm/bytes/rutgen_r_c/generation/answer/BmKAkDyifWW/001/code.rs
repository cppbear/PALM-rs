// Answer 0

#[test]
fn test_copy_to_bytes_valid_length() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(bytes_mut.len(), 5);

    let bytes = bytes_mut.copy_to_bytes(3);
    assert_eq!(bytes.len(), 3);
}

#[test]
#[should_panic(expected = "split_to out of bounds")]
fn test_copy_to_bytes_exceeding_length() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);

    // This should panic because we are trying to copy more bytes than available.
    let _ = bytes_mut.copy_to_bytes(6);
}

#[test]
fn test_copy_to_bytes_zero_length() {
    let mut bytes_mut = BytesMut::new();
    let bytes = bytes_mut.copy_to_bytes(0);
    assert_eq!(bytes.len(), 0);
}

#[test]
fn test_copy_to_bytes_empty() {
    let mut bytes_mut = BytesMut::new();
    let bytes = bytes_mut.copy_to_bytes(0);
    assert!(bytes.is_empty());
}

