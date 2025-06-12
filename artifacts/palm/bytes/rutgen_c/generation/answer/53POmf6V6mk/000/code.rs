// Answer 0

#[test]
fn test_extend_from_slice_with_initial_capacity() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b"abc");
    assert_eq!(b"abc", buf.as_slice());
}

#[test]
fn test_extend_from_slice_with_resizing() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b"abcde");
    buf.extend_from_slice(b"fghijkl");
    assert_eq!(b"abcdefghijkl", buf.as_slice());
}

#[test]
fn test_extend_from_slice_empty() {
    let mut buf = BytesMut::new();
    buf.extend_from_slice(b"");
    assert_eq!(b"", buf.as_slice());
}

#[test]
fn test_extend_from_slice_large_input() {
    let mut buf = BytesMut::with_capacity(10);
    let data = b"0123456789abcdefghijklmnopqrstuvwxyz";
    buf.extend_from_slice(data);
    assert_eq!(data, buf.as_slice());
}

#[test]
fn test_extend_from_slice_multiple_calls() {
    let mut buf = BytesMut::new();
    buf.extend_from_slice(b"foo");
    buf.extend_from_slice(b"bar");
    assert_eq!(b"foobar", buf.as_slice());
}

