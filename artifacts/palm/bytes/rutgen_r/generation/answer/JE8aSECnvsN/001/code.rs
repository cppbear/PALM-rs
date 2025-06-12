// Answer 0

#[test]
fn test_try_reclaim_success_when_additional_equals_rem() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    assert_eq!(true, buf.try_reclaim(64)); // additional == rem
    assert_eq!(64, buf.capacity());
}

#[test]
fn test_try_reclaim_success_when_additional_less_than_rem() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    assert_eq!(true, buf.try_reclaim(32)); // additional < rem
    assert_eq!(64, buf.capacity());
}

#[test]
fn test_try_reclaim_fails_when_reclaimed_buffer_has_active_references() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcd");
    let mut split = buf.split();
    assert_eq!(false, split.try_reclaim(64)); // Attempt to reclaim when there are active references
    assert_eq!(false, buf.try_reclaim(64));  // Same for original buffer
}

#[test]
fn test_try_reclaim_success_after_clearing_split() {
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcd");
    let mut split = buf.split();
    split.clear(); // Should clear the references
    assert_eq!(true, split.try_reclaim(64)); // Now should succeed after clear
    assert_eq!(64, split.capacity());
}

