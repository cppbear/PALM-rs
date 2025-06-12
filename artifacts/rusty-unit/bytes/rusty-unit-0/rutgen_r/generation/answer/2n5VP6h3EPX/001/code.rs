// Answer 0

#[test]
fn test_split_removes_bytes_and_returns_new_handle() {
    use bytes::{BytesMut, BufMut};

    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(1024, buf.capacity());
    assert_eq!(other, b"hello world"[..]);
}

#[test]
fn test_split_on_empty_buffer() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(0, buf.capacity());
    assert_eq!(other.len(), 0);
}

#[test]
fn test_split_with_capacity() {
    use bytes::{BytesMut, BufMut};

    let mut buf = BytesMut::with_capacity(512);
    buf.put(&b"test string"[..]);

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(512, buf.capacity());
    assert_eq!(other, b"test string"[..]);
}

#[test]
fn test_split_multiple_calls() {
    use bytes::{BytesMut, BufMut};

    let mut buf = BytesMut::with_capacity(256);
    buf.put(&b"split this"[..]);

    let other = buf.split();
    assert!(buf.is_empty());
    assert_eq!(256, buf.capacity());
    assert_eq!(other, b"split this"[..]);

    let second_other = buf.split();
    assert!(buf.is_empty());
    assert_eq!(256, buf.capacity());
    assert_eq!(second_other.len(), 0);
}

#[test]
#[should_panic]
fn test_split_panic_invalid_state() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    buf.split(); // This should not panic per design but it's good to ensure that it handles invalid states gracefully
}

