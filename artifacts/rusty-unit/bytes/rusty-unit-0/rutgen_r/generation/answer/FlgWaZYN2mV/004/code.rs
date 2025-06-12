// Answer 0

#[test]
fn test_split_off_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_off(0);

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello world");
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let _b = a.split_off(12); // at > len()
}

#[test]
fn test_split_off_full_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_off(11); // at == len()

    assert_eq!(&a[..], b"hello world");
    assert_eq!(&b[..], b"");
}

