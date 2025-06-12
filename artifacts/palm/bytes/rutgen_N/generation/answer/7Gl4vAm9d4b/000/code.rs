// Answer 0

#[test]
fn test_resize_increase_length() {
    let mut buf = bytes::BytesMut::new();
    buf.resize(3, 0x1);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_decrease_length() {
    let mut buf = bytes::BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(2, 0x2);
    assert_eq!(&buf[..], &[0x1, 0x1]);
}

#[test]
fn test_resize_increase_length_with_different_value() {
    let mut buf = bytes::BytesMut::new();
    buf.resize(2, 0x1);
    buf.resize(4, 0x3);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x3, 0x3]);
}

#[test]
fn test_resize_to_same_length() {
    let mut buf = bytes::BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(3, 0x2);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_to_zero() {
    let mut buf = bytes::BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(0, 0x2);
    assert_eq!(&buf[..], &[]);
}

#[test]
fn test_resize_from_large_to_small() {
    let mut buf = bytes::BytesMut::new();
    buf.resize(5, 0x1);
    buf.resize(3, 0x2);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_small_to_large_needs_capacity() {
    let mut buf = bytes::BytesMut::with_capacity(2);
    buf.resize(1, 0x1);
    assert_eq!(&buf[..], &[0x1]);
    buf.resize(4, 0x2);
    assert_eq!(&buf[..], &[0x1, 0x2, 0x2, 0x2]);
}

