// Answer 0

#[test]
fn test_bytes_mut_is_empty_true() {
    use bytes::BytesMut;

    let b = BytesMut::with_capacity(64);
    assert!(b.is_empty());
}

#[test]
fn test_bytes_mut_is_empty_false() {
    use bytes::BytesMut;

    let mut b = BytesMut::with_capacity(64);
    b.extend_from_slice(&[1, 2, 3]); // Add data to make it non-empty
    assert!(!b.is_empty());
}

