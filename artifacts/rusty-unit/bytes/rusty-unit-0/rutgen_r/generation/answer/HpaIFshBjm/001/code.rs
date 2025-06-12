// Answer 0

#[test]
fn test_truncate_exact_length() {
    use bytes::BytesMut;

    // Initialize with a buffer of "hello"
    let mut buf = BytesMut::from(&b"hello"[..]);
    let len = buf.len(); // len equals to the current length of the buffer
    buf.truncate(len);
    assert_eq!(buf, b"hello"[..]);
}

#[test]
fn test_truncate_with_zero_length() {
    use bytes::BytesMut;

    // Initialize with a buffer of "hello"
    let mut buf = BytesMut::from(&b"hello"[..]);
    buf.truncate(0);
    assert_eq!(buf, b""[..]);
}

#[test]
fn test_truncate_with_length_longer_than_buffer() {
    use bytes::BytesMut;

    // Initialize with a buffer of "hello"
    let mut buf = BytesMut::from(&b"hello"[..]);
    buf.truncate(10); // len is longer than current length, should have no effect
    assert_eq!(buf, b"hello"[..]);
}

#[test]
fn test_truncate_with_current_length() {
    use bytes::BytesMut;

    // Initialize with a buffer of "world"
    let mut buf = BytesMut::from(&b"world"[..]);
    let len = buf.len(); // get the current buffer length
    buf.truncate(len); // should not change the buffer
    assert_eq!(buf, b"world"[..]);
}

