// Answer 0

#[test]
fn test_truncate_no_change() {
    let mut buf = Bytes::from_static(b"hello");
    let initial_len = buf.len();
    buf.truncate(initial_len);
    assert_eq!(buf.len(), initial_len);
    assert_eq!(buf.copy_from_slice(b"hello"), buf);
}

#[test]
fn test_truncate_greater_than_len() {
    let mut buf = Bytes::from_static(b"hello");
    let initial_len = buf.len();
    buf.truncate(initial_len + 10);
    assert_eq!(buf.len(), initial_len);
    assert_eq!(buf.copy_from_slice(b"hello"), buf);
}

#[test]
fn test_truncate_to_zero() {
    let mut buf = Bytes::from_static(b"hello");
    buf.truncate(0);
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
}

#[test]
fn test_truncate_to_len() {
    let mut buf = Bytes::from_static(b"hello");
    buf.truncate(5);
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.copy_from_slice(b"hello"), buf);
}

