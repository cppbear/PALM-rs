// Answer 0

#[test]
fn test_split_off_at_capacity() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    let b = a.split_off(5);

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b"");
}

#[test]
fn test_split_off_at_zero() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    let b = a.split_off(0);

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello");
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    let _ = a.split_off(6); // This should panic as 6 > capacity.
}

