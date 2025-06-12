// Answer 0

#[test]
fn test_split_off_middle() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_off(5);

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b" world");
}

#[test]
fn test_split_off_empty() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b""[..]);
    let b = a.split_off(0);

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"");
}

#[test]
fn test_split_off_entire_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"full"[..]);
    let b = a.split_off(4);

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"full");
}

#[should_panic]
#[test]
fn test_split_off_out_of_bounds() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"test"[..]);
    let _ = a.split_off(5);
}

#[test]
fn test_split_off_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"split"[..]);
    let b = a.split_off(0);

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"split");
}

