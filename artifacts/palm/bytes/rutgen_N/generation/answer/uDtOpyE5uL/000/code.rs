// Answer 0

#[test]
fn test_with_capacity_zero() {
    let bytes = BytesMut::with_capacity(0);
    assert_eq!(bytes.len(), 0);
    assert!(bytes.capacity() >= 0);
}

#[test]
fn test_with_capacity_non_zero() {
    let bytes = BytesMut::with_capacity(64);
    assert_eq!(bytes.len(), 0);
    assert!(bytes.capacity() >= 64);
}

#[test]
fn test_with_capacity_exceeding_initial_capacity() {
    let mut bytes = BytesMut::with_capacity(10);
    assert_eq!(bytes.len(), 0);
    
    bytes.put(&b"hello world"[..]);
    
    assert_eq!(&bytes[..], b"hello world");
    assert!(bytes.capacity() >= 10);
}

