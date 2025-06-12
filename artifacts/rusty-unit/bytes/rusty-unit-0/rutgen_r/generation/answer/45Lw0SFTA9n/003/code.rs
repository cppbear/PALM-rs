// Answer 0

#[test]
fn test_split_to_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_to(0);

    assert_eq!(&a[..], b"hello world");
    assert_eq!(&b[..], b"");
}

#[test]
fn test_split_to_full_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_to(11); // at equals the length of the original bytes

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello world");
}

#[test]
#[should_panic]
fn test_split_to_panic_exceed_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let _b = a.split_to(12); // at exceeds the length of the original bytes
}

#[test]
fn test_split_to_non_empty() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_to(5); // at is less than the length of the original bytes

    assert_eq!(&a[..], b" world");
    assert_eq!(&b[..], b"hello");
}

