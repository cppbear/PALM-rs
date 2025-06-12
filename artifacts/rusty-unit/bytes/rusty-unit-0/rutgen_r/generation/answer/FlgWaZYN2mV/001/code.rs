// Answer 0

#[test]
fn test_split_off_at_equal_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_off(11); // Here at == self.len()

    assert_eq!(&a[..], b"hello world");
    assert!(b.is_empty());
}

#[test]
#[should_panic]
fn test_split_off_at_greater_than_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let _ = a.split_off(6); // This will panic as at > len
}

#[test]
fn test_split_off_at_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_off(0); // Here at == 0

    assert!(a.is_empty());
    assert_eq!(&b[..], b"hello");
}

