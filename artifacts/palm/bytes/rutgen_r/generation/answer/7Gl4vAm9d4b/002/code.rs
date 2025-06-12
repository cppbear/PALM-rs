// Answer 0

#[test]
fn test_resize_increase_length() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    buf.resize(5, 0x1); // Increase length, should fill with 0x1
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1, 0x1, 0x1]); // Expecting five 0x1 bytes
}

#[test]
fn test_resize_increase_length_with_different_value() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    buf.resize(3, 0x2); // Increase length with a different fill value
    assert_eq!(&buf[..], &[0x2, 0x2, 0x2]); // Expecting three 0x2 bytes
}

#[test]
fn test_resize_increase_length_exceeding_previous_length() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"[0x1, 0x1]"[..]);
    buf.resize(6, 0x3); // Increase length, initial length is 2
    assert_eq!(&buf[..], &[0x1, 0x1, 0x3, 0x3, 0x3, 0x3]); // Expecting two 0x1 and four 0x3
}

#[test]
fn test_resize_decrease_length() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"[0x1, 0x1, 0x1, 0x1, 0x1]"[..]);
    buf.resize(3, 0x2); // Decrease length, should remove the last two
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]); // Expecting three 0x1 bytes, no change
}

#[test]
fn test_resize_truncate_length() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"[0x1, 0x1, 0x1, 0x3, 0x3]"[..]);
    buf.resize(2, 0x4); // Truncate to 2, should keep the first two
    assert_eq!(&buf[..], &[0x1, 0x1]); // Expecting two 0x1 bytes
}

