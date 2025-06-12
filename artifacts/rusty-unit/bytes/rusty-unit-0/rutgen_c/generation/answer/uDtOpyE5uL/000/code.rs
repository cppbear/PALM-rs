// Answer 0

#[test]
fn test_bytes_mut_with_capacity_zero() {
    let bytes = BytesMut::with_capacity(0);
    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.capacity(), 0);
}

#[test]
fn test_bytes_mut_with_capacity_non_zero() {
    let capacity = 64;
    let bytes = BytesMut::with_capacity(capacity);
    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.capacity(), capacity);
}

#[test]
fn test_bytes_mut_with_capacity_large() {
    let capacity = usize::MAX;
    let bytes = BytesMut::with_capacity(capacity);
    assert_eq!(bytes.len(), 0);
    assert!(bytes.capacity() >= capacity);
}

