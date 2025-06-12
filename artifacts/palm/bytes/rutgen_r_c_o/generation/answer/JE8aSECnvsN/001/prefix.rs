// Answer 0

#[test]
fn test_try_reclaim_exact_rem() {
    let mut buf = BytesMut::with_capacity(64);
    assert!(buf.try_reclaim(64));
}

#[test]
fn test_try_reclaim_zero_additional() {
    let mut buf = BytesMut::with_capacity(20);
    buf.extend_from_slice(b"test");
    assert!(buf.try_reclaim(0));
}

#[test]
fn test_try_reclaim_same_as_rem() {
    let mut buf = BytesMut::with_capacity(60);
    buf.extend_from_slice(b"hello world");
    let remaining_capacity = buf.capacity() - buf.len();
    assert!(buf.try_reclaim(remaining_capacity));
}

#[test]
fn test_try_reclaim_full_capacity() {
    let mut buf = BytesMut::with_capacity(32);
    assert!(buf.try_reclaim(32));
    buf.extend_from_slice(&[0u8; 32]);
    assert!(buf.try_reclaim(0));
}

