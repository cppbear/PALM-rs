// Answer 0

#[test]
fn test_split_off_valid_index() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello world"[..]);
    let mut b = a.split_off(5);

    a[0] = b'j';
    b[0] = b'!';

    assert_eq!(&a[..], b"jello");
    assert_eq!(&b[..], b"!world");
}

#[test]
#[should_panic(expected = "split_off out of bounds: 15 <= 11")]
fn test_split_off_out_of_bounds() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    a.split_off(15);
} 

#[test]
fn test_split_off_zero_index() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    let mut b = a.split_off(0);

    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello");
}

#[test]
fn test_split_off_full_capacity() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    let mut b = a.split_off(5);

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b"");
}

