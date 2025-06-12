// Answer 0

#[test]
fn test_split_to_equal_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_to(a.len());

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello");
}

#[test]
#[should_panic]
fn test_split_to_exceed_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let _b = a.split_to(6); // This should panic as 6 > length of "hello"
}

#[test]
fn test_split_to_zero_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_to(0);

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b"");
}

