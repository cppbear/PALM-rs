// Answer 0

#[test]
fn test_reserve_with_large_additional_allocation() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    buf.put(&[0; 32][..]);

    assert_eq!(buf.len(), 32);
    assert_eq!(buf.capacity(), 64);

    // additional > rem (64 - 32 = 32, so additional must be greater than 32)
    buf.reserve(33);

    assert!(buf.capacity() >= 97); // New capacity should be at least 32 + 33 + some slack
}

#[test]
fn test_reserve_from_empty_buf() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 64);

    // additional > rem (rem is 64 here as buf is empty)
    buf.reserve(65);

    assert!(buf.capacity() >= 129); // New capacity should reflect the additional request
}

#[should_panic]
fn test_reserve_with_overflow() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(usize::MAX);
    
    // This is a boundary condition that will cause a panic because additional will overflow
    buf.reserve(1);
}

