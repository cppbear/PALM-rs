// Answer 0

#[test]
fn test_split_non_empty_buffer() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(&b"hello world"[..]);

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 1024);
    assert_eq!(other.as_slice(), b"hello world"[..]);
}

#[test]
fn test_split_empty_buffer() {
    let mut buf = BytesMut::new();

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 0);
    assert_eq!(other.len(), 0);
}

#[test]
fn test_split_with_capacity() {
    let mut buf = BytesMut::with_capacity(512);
    buf.extend_from_slice(&b"test string"[..]);

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 512);
    assert_eq!(other.len(), 11);
    assert_eq!(other.as_slice(), b"test string"[..]);
}

#[test]
#[should_panic]
fn test_split_panic_on_empty() {
    let mut buf = BytesMut::new();
    buf.split_to(1); // This should panic as there are not enough bytes
}

