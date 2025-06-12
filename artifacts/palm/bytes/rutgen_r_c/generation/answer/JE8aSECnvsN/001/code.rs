// Answer 0

#[test]
fn test_try_reclaim_success_when_additional_equals_rem() {
    let mut buf = BytesMut::with_capacity(64);
    assert_eq!(true, buf.try_reclaim(64)); // Initial try_reclaim should succeed
    assert_eq!(64, buf.capacity()); // Capacity should still be 64

    buf.extend_from_slice(b"abcd"); // Extend buffer with some data
    let mut split = buf.split(); // Split the buffer
    assert_eq!(60, buf.capacity()); // Remaining capacity of the first buffer
    assert_eq!(4, split.capacity()); // Capacity of the split

    assert_eq!(false, split.try_reclaim(64)); // Should fail as it needs more capacity
    assert_eq!(false, buf.try_reclaim(64)); // Should also fail for the same reason

    assert_eq!(true, buf.try_reclaim(60)); // buf empty should reclaim 60 successfully
    assert_eq!(true, split.try_reclaim(4)); // split still has space for 4 bytes
}

#[test]
fn test_try_reclaim_true_on_empty_buffer() {
    let mut buf = BytesMut::new(); // Create a new empty buffer
    // At this point, capacity is 0
    assert_eq!(true, buf.try_reclaim(0)); // Should always succeed if nothing is requested
}

#[test]
fn test_try_reclaim_exceeds_capacity() {
    let mut buf = BytesMut::with_capacity(16);
    assert_eq!(true, buf.try_reclaim(16)); // Should succeed as we have enough capacity

    buf.extend_from_slice(b"abcdefgh"); // Fill buffer to some length
    assert_eq!(8, buf.len()); // Length is now 8
    assert_eq!(8, buf.capacity()); // Ensure capacity is still enough

    assert_eq!(false, buf.try_reclaim(16)); // Should fail as additional exceeds rem
}

#[test]
fn test_try_reclaim_after_clear() {
    let mut buf = BytesMut::with_capacity(32);
    assert_eq!(true, buf.try_reclaim(32)); // Initial reclaim should succeed
    buf.extend_from_slice(b"Hello, World!"); // Fill the buffer with data
    assert_eq!(false, buf.try_reclaim(32)); // Not enough capacity to reclaim

    buf.clear(); // Clear the buffer
    assert_eq!(true, buf.try_reclaim(32)); // Should succeed after clearing
}

