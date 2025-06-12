// Answer 0

#[test]
fn test_truncate_with_exact_length() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello world"[..]);
    buf.truncate(11);
    assert_eq!(buf, b"hello world"[..]);
}

#[test]
fn test_truncate_with_shorter_length() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello world"[..]);
    buf.truncate(5);
    assert_eq!(buf, b"hello"[..]);
}

#[test]
fn test_truncate_with_length_zero() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello world"[..]);
    buf.truncate(0);
    assert_eq!(buf, b""[..]);
}

#[test]
fn test_truncate_with_length_greater_than_current_length() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello"[..]);
    buf.truncate(10);
    assert_eq!(buf, b"hello"[..]);
}

