// Answer 0

#[test]
fn test_extend_from_slice_with_zero_capacity() {
    let mut buf = BytesMut::with_capacity(0);
    buf.extend_from_slice(b"");
    assert_eq!(b"", &buf as &[u8]);
}

#[test]
fn test_extend_from_slice_with_exact_capacity() {
    let mut buf = BytesMut::with_capacity(6);
    buf.extend_from_slice(b"aaaaaa");
    assert_eq!(b"aaaaaa", &buf as &[u8]);
}

#[test]
fn test_extend_from_slice_growing_capacity() {
    let mut buf = BytesMut::with_capacity(3);
    buf.extend_from_slice(b"aaa");
    assert_eq!(b"aaa", &buf as &[u8]);
    buf.extend_from_slice(b"bb");
    assert_eq!(b"aabbb", &buf as &[u8]);
}

#[test]
#[should_panic]
fn test_extend_from_slice_exceeding_capacity_properly_resizes() {
    let mut buf = BytesMut::with_capacity(4);
    // Reserve space that will trigger a resize when we extend
    buf.extend_from_slice(b"hello world"); // expecting panic due to resizing
}

#[test]
fn test_extend_from_slice_boundary_condition() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b"1");
    buf.extend_from_slice(b"234");
    assert_eq!(b"1234", &buf as &[u8]);
    buf.extend_from_slice(b"56");
    assert_eq!(b"123456", &buf as &[u8]);
}

