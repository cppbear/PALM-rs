// Answer 0

#[test]
fn test_resize_increase_length() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(3, 0x1);
    assert_eq!(buf.len(), 3);
    assert_eq!(&buf.spare_capacity_mut()[..3], &[0x1, 0x1, 0x1]);

    buf.resize(5, 0x2);
    assert_eq!(buf.len(), 5);
    assert_eq!(&buf.spare_capacity_mut()[..5], &[0x1, 0x1, 0x2, 0x2, 0x2]);
}

#[test]
fn test_resize_decrease_length() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(4, 0x1);
    assert_eq!(buf.len(), 4);
    assert_eq!(&buf.spare_capacity_mut()[..4], &[0x1, 0x1, 0x1, 0x1]);

    buf.resize(2, 0x2);
    assert_eq!(buf.len(), 2);
    assert_eq!(&buf.spare_capacity_mut()[..2], &[0x1, 0x1]);
}

#[test]
fn test_resize_boundary_case() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(0, 0x1);
    assert_eq!(buf.len(), 0);

    buf.resize(3, 0x3);
    assert_eq!(buf.len(), 3);
    assert_eq!(&buf.spare_capacity_mut()[..3], &[0x3, 0x3, 0x3]);

    buf.resize(3, 0x4);
    assert_eq!(buf.len(), 3);
    assert_eq!(&buf.spare_capacity_mut()[..3], &[0x3, 0x3, 0x3]);
}

