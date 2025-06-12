// Answer 0

#[test]
fn test_copy_to_bytes() {
    use bytes::{Bytes, BytesMut};
    
    let mut bytes_mut = BytesMut::from(&b"Hello, World!"[..]);
    let copied_bytes = bytes_mut.copy_to_bytes(5);

    assert_eq!(copied_bytes, Bytes::from(&b"Hello"[..]));
    assert_eq!(bytes_mut, BytesMut::from(&b", World!"[..]));
}

#[test]
#[should_panic]
fn test_copy_to_bytes_panic_too_large() {
    use bytes::{Bytes, BytesMut};

    let mut bytes_mut = BytesMut::from(&b"Hello"[..]);
    let _ = bytes_mut.copy_to_bytes(10);
}

