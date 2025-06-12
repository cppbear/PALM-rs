// Answer 0

#[test]
fn test_resize_increase_length() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    assert_eq!(buf.len(), 3);
    assert_eq!(&buf.spare_capacity_mut()[..3], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_truncate() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(2, 0x2);
    assert_eq!(buf.len(), 2);
    assert_eq!(&buf.spare_capacity_mut()[..2], &[0x1, 0x1]);
}

#[test]
fn test_resize_no_change() {
    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    buf.resize(3, 0x2); // new_len == len, should not change
    assert_eq!(buf.len(), 3);
    assert_eq!(&buf.spare_capacity_mut()[..3], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_with_value() {
    let mut buf = BytesMut::new();
    buf.resize(2, 0x1);
    buf.resize(4, 0x2); // increasing with new value
    assert_eq!(buf.len(), 4);
    assert_eq!(&buf.spare_capacity_mut()[..4], &[0x1, 0x1, 0x2, 0x2]);
}

