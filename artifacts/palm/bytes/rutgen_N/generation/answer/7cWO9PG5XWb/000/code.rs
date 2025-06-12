// Answer 0

#[test]
fn test_reserve_increases_capacity() {
    use bytes::BytesMut;

    let mut buf = BytesMut::from(&b"hello"[..]);
    buf.reserve(64);
    assert!(buf.capacity() >= 69); // 5 (original) + 64 (requested)
}

#[test]
fn test_reserve_reclaims_space() {
    use bytes::{BytesMut, BufMut};

    let mut buf = BytesMut::with_capacity(128);
    buf.put(&[0; 64][..]);

    let ptr = buf.as_ptr();
    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 64);

    drop(other);
    buf.reserve(128);

    assert_eq!(buf.capacity(), 128);
    assert_eq!(buf.as_ptr(), ptr);
}

#[should_panic]
fn test_reserve_panics_on_overflow() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(0);
    buf.reserve(usize::MAX);
}

