// Answer 0

#[test]
fn test_advance_mut_with_sufficient_capacity() {
    let mut buffer = vec![0u8; 5]; // Initial buffer of size 5
    let capacity = buffer.capacity(); // Should be at least 5

    unsafe {
        buffer.advance_mut(3); // Advance by 3
    }

    assert_eq!(buffer.len(), 3); // Length should now be 3
    assert_eq!(buffer.capacity(), capacity); // Capacity should remain the same
}

#[test]
#[should_panic(expected = "advance out of bounds")]
fn test_advance_mut_with_insufficient_capacity() {
    let mut buffer = vec![0u8; 5]; // Initial buffer of size 5
    unsafe {
        buffer.set_len(5); // Set length to maximum
        buffer.advance_mut(1); // Try to advance by 1, should panic
    }
}

#[test]
fn test_advance_mut_edge_case() {
    let mut buffer = vec![0u8; 5]; // Initial buffer of size 5
    unsafe {
        buffer.set_len(0); // Set length to 0
        buffer.advance_mut(5); // Advance by 5, should not panic if capacity allows it
    }

    assert_eq!(buffer.len(), 5); // Length should now be 5
}

