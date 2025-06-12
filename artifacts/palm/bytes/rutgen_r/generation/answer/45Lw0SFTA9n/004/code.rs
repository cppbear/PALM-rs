// Answer 0

#[test]
fn test_split_to_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_to(0);
    
    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b"");
}

#[test]
#[should_panic]
fn test_split_to_out_of_bounds() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let _b = a.split_to(6); // at > len
}

#[test]
fn test_split_to_non_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_to(5);
    
    assert_eq!(&a[..], b" world");
    assert_eq!(&b[..], b"hello");
}

