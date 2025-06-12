// Answer 0

#[test]
fn test_split_off_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_off(0);

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello");
}

#[test]
fn test_split_off_full_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"world"[..]);
    let b = a.split_off(5);

    assert_eq!(&a[..], b"world");
    assert_eq!(&b[..], b"");
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"rust"[..]);
    let _b = a.split_off(5);
}

#[test]
fn test_split_off_mid_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"example"[..]);
    let b = a.split_off(4);

    assert_eq!(&a[..], b"exam");
    assert_eq!(&b[..], b"ple");
}

