// Answer 0

#[test]
fn test_try_reclaim_success_no_allocation() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    assert_eq!(true, buf.try_reclaim(64));
    assert_eq!(64, buf.capacity());
}

#[test]
fn test_try_reclaim_fail_with_split() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcd");
    let mut split = buf.split();
    
    assert_eq!(60, buf.capacity());
    assert_eq!(4, split.capacity());
    assert_eq!(false, split.try_reclaim(64));
    assert_eq!(false, buf.try_reclaim(64));
    assert_eq!(false, split.try_reclaim(4));
}

#[test]
fn test_try_reclaim_success_empty_buf() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    let mut split = buf.split();

    drop(buf);
    assert_eq!(false, split.try_reclaim(64));
    
    split.clear();
    assert_eq!(4, split.capacity());
    assert_eq!(true, split.try_reclaim(64));
    assert_eq!(64, split.capacity());
}

