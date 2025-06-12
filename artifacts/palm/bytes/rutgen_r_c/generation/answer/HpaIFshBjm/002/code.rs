// Answer 0

#[test]
fn test_truncate_valid_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    buf.truncate(3);
    assert_eq!(buf.len(), 3);
}

#[test]
fn test_truncate_greater_than_length() {
    let mut buf = BytesMut::from(vec![1, 2, 3]);
    buf.truncate(5);
    assert_eq!(buf.len(), 3);
}

#[test]
#[should_panic]
fn test_truncate_zero_length() {
    let mut buf = BytesMut::from(vec![1, 2, 3]);
    buf.truncate(0);
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_truncate_equal_length() {
    let mut buf = BytesMut::from(vec![1, 2, 3]);
    buf.truncate(3);
    assert_eq!(buf.len(), 3);
}

#[test]
fn test_truncate_at_full_capacity() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    buf.truncate(5);
    assert_eq!(buf.len(), 5);
}

