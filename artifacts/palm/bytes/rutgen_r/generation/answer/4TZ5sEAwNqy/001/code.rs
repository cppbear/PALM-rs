// Answer 0

#[test]
fn test_clear_non_empty_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello world"[..]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_empty_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_large_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(vec![0u8; 1024]); // Large buffer
    assert_eq!(buf.len(), 1024);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_buffer_with_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(&b"data"[..]);
    assert!(!buf.is_empty());
    buf.clear();
    assert!(buf.is_empty());
}

#[should_panic]
fn test_clear_on_buffer_with_invalid_state() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello"[..]);
    unsafe {
        buf.set_len(10); // Intentionally set to an invalid length
    }
    buf.clear(); // This should panic due to invalid length
}

