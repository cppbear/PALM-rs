// Answer 0

#[test]
fn test_truncate_equal_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(b"hello");
    let original_len = buf.len();
    buf.truncate(original_len);
    assert_eq!(buf.len(), original_len);
    assert_eq!(buf.as_slice(), b"hello"[..]);
}

#[test]
fn test_truncate_zero_length() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    buf.truncate(0);
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
}

#[test]
fn test_truncate_with_inner_capacity() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(b"hello world");
    let original_len = buf.len();
    buf.truncate(original_len);
    assert_eq!(buf.len(), original_len);
    assert_eq!(buf.as_slice(), b"hello world"[..]);
}

#[test]
#[should_panic]
fn test_truncate_beyond_length() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3]);
    buf.truncate(5); // This should not panic as per logic, but tests overflow.
}

#[test]
fn test_truncate_half_length() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    buf.truncate(2);
    assert_eq!(buf.len(), 2);
    assert_eq!(buf.as_slice(), &[1, 2]);
}

#[test]
fn test_truncate_to_length_of_one() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3]);
    buf.truncate(1);
    assert_eq!(buf.len(), 1);
    assert_eq!(buf.as_slice(), &[1]);
}

