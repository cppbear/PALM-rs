// Answer 0

#[test]
fn test_clear_empty_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(10);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_non_empty_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello world"[..]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_buffer_with_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"data"[..]);
    assert!(!buf.is_empty());
    assert!(buf.capacity() >= 4); // check initial capacity is preserved
    buf.clear();
    assert!(buf.is_empty());
    assert!(buf.capacity() >= 4); // ensure capacity is still preserved after clear
}

