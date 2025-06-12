// Answer 0

#[test]
fn test_resize_increase_length() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    assert_eq!(buf.len(), 3);
    assert_eq!(&buf.as_slice()[..], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_decrease_length() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(2, 0x2);
    assert_eq!(buf.len(), 2);
    assert_eq!(&buf.as_slice()[..], &[0x1, 0x1]);
}

#[test]
fn test_resize_increase_length_with_different_value() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(4, 0x3);
    assert_eq!(buf.len(), 4);
    assert_eq!(&buf.as_slice()[..], &[0x1, 0x1, 0x3, 0x3]);
}

#[test]
fn test_resize_to_same_length() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(3, 0x2); // Should not change the buffer
    assert_eq!(buf.len(), 3);
    assert_eq!(&buf.as_slice()[..], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_truncate_to_zero() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(0, 0x2);
    assert_eq!(buf.len(), 0);
}

