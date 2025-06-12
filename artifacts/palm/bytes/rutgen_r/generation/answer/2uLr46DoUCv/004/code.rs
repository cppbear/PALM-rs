// Answer 0

#[test]
fn test_truncate_equal_length() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello"[..]);
    buf.truncate(5);
    assert_eq!(buf, b"hello"[..]);
}

#[test]
fn test_truncate_greater_length() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello"[..]);
    buf.truncate(10);
    assert_eq!(buf, b"hello"[..]);
}

#[test]
fn test_truncate_zero_length() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello"[..]);
    buf.truncate(0);
    assert_eq!(buf.len(), 0);
    // For full verification, check if buf is empty
    assert!(buf.is_empty());
}

#[test]
fn test_truncate_with_len_equals_capacity() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello"[..]);
    buf.truncate(5); // The length equals the current length
    assert_eq!(buf.len(), 5);
    assert_eq!(buf, b"hello"[..]);
}

#[test]
#[should_panic]
fn test_truncate_with_len_greater_than_initial_length() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hi"[..]);
    buf.truncate(5); // This should panic as it violates the assumption
}

