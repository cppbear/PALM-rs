// Answer 0

#[test]
fn test_advance_exact_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5; cap is 10

    // Check remaining is 5 and advancing by 5 should not panic
    let cnt = bytes_mut.remaining(); // Should be 5
    bytes_mut.advance(cnt);
    assert_eq!(bytes_mut.len(), 0); // length after advancing should be 0
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`")]
fn test_advance_exceeding_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5; cap is 10

    // Attempt to advance beyond the remaining length
    let cnt = bytes_mut.remaining() + 1; // Should be 6
    bytes_mut.advance(cnt);
}

#[test]
fn test_advance_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5; cap is 10

    // Advancing by 0 should be a no-op
    bytes_mut.advance(0);
    assert_eq!(bytes_mut.len(), 5); // Length should remain unchanged
    assert_eq!(bytes_mut.remaining(), 5); // Remaining should still be 5
}

#[test]
fn test_advance_full_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0); // Set len to 10; cap is 10

    // Advancing by the full capacity
    let cnt = bytes_mut.remaining(); // Should be 10
    bytes_mut.advance(cnt);
    assert_eq!(bytes_mut.len(), 0); // Length after advancing should be 0
    assert_eq!(bytes_mut.remaining(), 0); // Remaining should also be 0
}

