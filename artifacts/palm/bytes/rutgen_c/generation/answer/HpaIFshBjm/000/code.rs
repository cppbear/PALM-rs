// Answer 0

#[test]
fn test_truncate_within_bounds() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    buf.truncate(5);
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_truncate_to_zero() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3]);
    buf.truncate(0);
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.is_empty(), true);
}

#[test]
fn test_truncate_exceeding_length() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3]);
    buf.truncate(5);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_truncate_equal_length() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3]);
    buf.truncate(3);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.as_slice(), &[1, 2, 3]);
}

