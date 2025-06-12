// Answer 0

#[test]
fn test_clear_buffer() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello world"[..]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_empty_buffer() {
    use bytes::Bytes;

    let mut buf = Bytes::new();
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_large_buffer() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"this is a large buffer containing a lot of data"[..]);
    buf.clear();
    assert!(buf.is_empty());
}

