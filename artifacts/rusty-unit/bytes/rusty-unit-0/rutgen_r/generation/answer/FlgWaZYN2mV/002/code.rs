// Answer 0

#[test]
fn test_split_off_non_zero_at() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_off(5);

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b" world");
}

#[test]
fn test_split_off_middle() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"Rust programming"[..]);
    let b = a.split_off(4);

    assert_eq!(&a[..], b"Rust");
    assert_eq!(&b[..], b" programming");
}

#[test]
#[should_panic]
fn test_split_off_at_length() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello world"[..]);
    let _b = a.split_off(11); // here at is length of the Bytes, should panic
}

#[test]
#[should_panic]
fn test_split_off_at_zero() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"hello"[..]);
    let _b = a.split_off(0); // here at is 0, should panic
}

#[test]
fn test_split_off_at_valid_middle() {
    use bytes::Bytes;

    let mut a = Bytes::from(&b"abcdefghi"[..]);
    let b = a.split_off(5);

    assert_eq!(&a[..], b"abcde");
    assert_eq!(&b[..], b"fghi");
}

