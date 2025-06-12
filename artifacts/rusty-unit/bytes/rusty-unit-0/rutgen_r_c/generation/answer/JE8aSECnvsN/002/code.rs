// Answer 0

#[test]
fn test_try_reclaim_insufficient_capacity() {
    let mut buf = BytesMut::with_capacity(64);
    // Initially, should have enough capacity to reclaim 64 bytes
    assert_eq!(true, buf.try_reclaim(64));
    assert_eq!(64, buf.capacity());

    buf.extend_from_slice(b"abcd");
    // Split the buffer, which will reduce the original buffer's capacity
    let mut split = buf.split();
    assert_eq!(60, buf.capacity());
    assert_eq!(4, split.capacity());

    // Trying to reclaim 64 bytes should fail
    assert_eq!(false, split.try_reclaim(64));
    assert_eq!(false, buf.try_reclaim(64));

    // The split buffer is filled with "abcd", which means it has few references
    assert_eq!(false, split.try_reclaim(4)); // Not enough space as it has 0 spare

    // buf is empty and has capacity for 60 bytes, can reclaim 60 
    assert_eq!(true, buf.try_reclaim(60));

    drop(buf); // Dropping buf should lead split being isolated
    
    // Post-drop, split should have insufficient capacity to reclaim
    assert_eq!(false, split.try_reclaim(64));
}

#[test]
fn test_try_reclaim_after_clear() {
    let mut buf = BytesMut::with_capacity(64);
    assert_eq!(true, buf.try_reclaim(64));
    assert_eq!(64, buf.capacity());

    buf.extend_from_slice(b"abcd");
    let mut split = buf.split();
    split.clear(); // Clear split buffer changing state

    // After clearing, it should succeed in reclaiming 64
    assert_eq!(4, split.capacity()); // the capacity remains the same despite being clear
    assert_eq!(true, split.try_reclaim(64));
    assert_eq!(64, split.capacity());
}

