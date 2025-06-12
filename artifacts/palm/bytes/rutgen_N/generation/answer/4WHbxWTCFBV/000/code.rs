// Answer 0

#[test]
fn test_put_bytes() {
    use std::ptr;
    use bytes::BytesMut;

    let mut bytes = BytesMut::with_capacity(10);
    
    // Test putting 5 bytes of value 1
    bytes.put_bytes(1, 5);
    assert_eq!(&bytes[..5], &[1, 1, 1, 1, 1]);
    assert_eq!(bytes.len(), 5);

    // Test putting 3 bytes of value 2
    bytes.put_bytes(2, 3);
    assert_eq!(&bytes[5..8], &[2, 2, 2]);
    assert_eq!(bytes.len(), 8);

    // Test putting 0 bytes
    bytes.put_bytes(3, 0);
    assert_eq!(bytes.len(), 8);  // Length should remain same
}

#[test]
fn test_put_bytes_boundary() {
    use std::ptr;
    use bytes::BytesMut;

    let mut bytes = BytesMut::with_capacity(5);

    // Test putting 5 bytes of value 4, which exactly matches the length of the buffer
    bytes.put_bytes(4, 5);
    assert_eq!(&bytes[..5], &[4, 4, 4, 4, 4]);
    assert_eq!(bytes.len(), 5);

    // Test putting 6 bytes which exceeds initial capacity
    bytes.put_bytes(5, 6);
    assert_eq!(&bytes[5..11], &[5, 5, 5, 5, 5, 5]);
    assert_eq!(bytes.len(), 11);
}

