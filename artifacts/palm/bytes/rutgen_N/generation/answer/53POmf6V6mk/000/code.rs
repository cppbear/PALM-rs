// Answer 0

#[test]
fn test_extend_from_slice_empty_initial_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(0);
    buf.extend_from_slice(b"hello");

    assert_eq!(b"hello", &buf[..]);
}

#[test]
fn test_extend_from_slice_non_empty_initial_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(b"world");

    assert_eq!(b"world", &buf[..]);
}

#[test]
fn test_extend_from_slice_multiple_calls() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(0);
    buf.extend_from_slice(b"foo");
    buf.extend_from_slice(b"bar");

    assert_eq!(b"foobar", &buf[..]);
}

#[test]
fn test_extend_from_slice_exceeding_initial_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(3);
    buf.extend_from_slice(b"abc");
    buf.extend_from_slice(b"defgh");

    assert_eq!(b"abcdefgh", &buf[..]);
}

#[test]
fn test_extend_from_slice_empty_slice() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b"test");
    buf.extend_from_slice(&[]); // extending with an empty slice

    assert_eq!(b"test", &buf[..]);
}

