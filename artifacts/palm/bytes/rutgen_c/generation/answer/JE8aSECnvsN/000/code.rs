// Answer 0

#[test]
fn test_try_reclaim_empty_buffer() {
    let mut buf = BytesMut::new();
    assert_eq!(true, buf.try_reclaim(0));
    assert_eq!(0, buf.capacity());
}

#[test]
fn test_try_reclaim_sufficient_capacity() {
    let mut buf = BytesMut::with_capacity(64);
    assert_eq!(true, buf.try_reclaim(64));
    assert_eq!(64, buf.capacity());
}

#[test]
fn test_try_reclaim_insufficient_capacity() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcd");
    assert_eq!(60, buf.capacity());
    assert_eq!(false, buf.try_reclaim(64));
}

#[test]
fn test_try_reclaim_after_split() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcd");
    let mut split = buf.split();
    assert_eq!(60, buf.capacity());
    assert_eq!(4, split.capacity());
    assert_eq!(false, split.try_reclaim(64));
}

#[test]
fn test_try_reclaim_empty_after_split() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcd");
    let mut split = buf.split();
    split.clear();
    assert_eq!(4, split.capacity());
    assert_eq!(true, split.try_reclaim(64));
    assert_eq!(64, split.capacity());
}

