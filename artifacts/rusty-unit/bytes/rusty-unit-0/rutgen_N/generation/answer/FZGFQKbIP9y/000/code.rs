// Answer 0

#[test]
fn test_set_len_in_bounds() {
    use bytes::BytesMut;

    let mut b = BytesMut::from(&b"hello world"[..]);

    unsafe {
        b.set_len(5);
    }

    assert_eq!(&b[..], b"hello");

    unsafe {
        b.set_len(11);
    }

    assert_eq!(&b[..], b"hello world");
}

#[test]
#[should_panic]
fn test_set_len_out_of_bounds() {
    use bytes::BytesMut;

    let mut b = BytesMut::from(&b"hello"[..]);

    unsafe {
        b.set_len(10); // This should panic due to out of bounds length
    }
}

