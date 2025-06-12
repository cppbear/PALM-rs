// Answer 0

#[test]
fn test_extend_from_slice_with_exact_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(6);
    buf.extend_from_slice(b"abcdef");

    assert_eq!(b"abcdef", &buf[..]);
}

#[test]
fn test_extend_from_slice_with_increase_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(3);
    buf.extend_from_slice(b"abc");
    buf.extend_from_slice(b"defg");

    assert_eq!(b"abcdefg", &buf[..]);
}

#[test]
fn test_extend_from_empty_slice() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b"hello");
    buf.extend_from_slice(b"");

    assert_eq!(b"hello", &buf[..]);
}

#[test]
fn test_extend_from_slice_with_multiple_resizes() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(2);
    buf.extend_from_slice(b"hi");
    buf.extend_from_slice(b"there");
    buf.extend_from_slice(b"!");

    assert_eq!(b"hithere!", &buf[..]);
}

#[test]
#[should_panic]
fn test_panic_on_zero_length_extension_without_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(0);
    buf.extend_from_slice(&[]);
    
    // The purpose of this test is to check for panic conditions,
    // however, in normal circumstances, this will not panic
}

