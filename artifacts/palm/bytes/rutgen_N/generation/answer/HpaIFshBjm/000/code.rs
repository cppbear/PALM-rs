// Answer 0

#[test]
fn test_truncate_with_smaller_len() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello world"[..]);
    buf.truncate(5);
    assert_eq!(buf, b"hello"[..]);
}

#[test]
fn test_truncate_with_equal_len() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello"[..]);
    buf.truncate(5);
    assert_eq!(buf, b"hello"[..]);
}

#[test]
fn test_truncate_with_greater_len() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hi"[..]);
    buf.truncate(10);  // No effect since len > current buffer length
    assert_eq!(buf, b"hi"[..]);
}

#[test]
fn test_truncate_with_zero_len() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello"[..]);
    buf.truncate(0);
    assert_eq!(buf.len(), 0);
}

