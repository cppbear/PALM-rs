// Answer 0

#[test]
fn test_unsplit_empty_self() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    let other = BytesMut::from("abc");
    buf.unsplit(other);
    assert_eq!(buf, BytesMut::from("abc"));
}

#[test]
fn test_unsplit_non_empty_self() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from("hello");
    let other = BytesMut::from(" world");
    buf.unsplit(other);
    assert_eq!(buf, BytesMut::from("hello world"));
}

#[test]
fn test_unsplit_split_case() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"hello world");
    
    let split = buf.split_off(6);
    assert_eq!(&buf[..], b"hello");
    assert_eq!(&split[..], b" world");

    buf.unsplit(split);
    assert_eq!(&buf[..], b"hello world");
}

