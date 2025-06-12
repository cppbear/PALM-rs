// Answer 0

#[test]
fn test_new_bytes_mut_empty() {
    let bytes = BytesMut::new();
}

#[test]
fn test_new_bytes_mut_zero_length() {
    let bytes = BytesMut::new();
}

#[test]
fn test_new_bytes_mut_no_allocation() {
    let bytes = BytesMut::new();
    let capacity = bytes.capacity();
}

#[test]
fn test_new_bytes_mut_default_capacity() {
    let bytes = BytesMut::new();
    assert_eq!(0, bytes.len());
}

