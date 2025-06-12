// Answer 0

#[test]
fn test_truncate_length_greater_than_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello world"[..]);
    buf.truncate(15);  // len is greater than current length (11)
    assert_eq!(buf, b"hello world"[..]);  // No change expected
}

#[test]
fn test_truncate_length_equals_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello"[..]);
    buf.truncate(5);  // len equals current length
    assert_eq!(buf, b"hello"[..]);  // No change expected
}

#[test]
fn test_truncate_length_less_than_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello world"[..]);
    buf.truncate(5);  // len is less than current length
    assert_eq!(buf, b"hello"[..]);  // Should truncate as expected
}

