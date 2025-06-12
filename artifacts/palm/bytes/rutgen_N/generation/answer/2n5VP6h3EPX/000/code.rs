// Answer 0

#[test]
fn test_split_non_empty() {
    use bytes::{BytesMut, BufMut};

    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(1014, buf.capacity());
    assert_eq!(other, b"hello world"[..]);
}

#[test]
fn test_split_empty() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(1024);

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(1024, buf.capacity());
    assert_eq!(other.len(), 0);
}

