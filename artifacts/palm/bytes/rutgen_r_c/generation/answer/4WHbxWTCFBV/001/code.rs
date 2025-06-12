// Answer 0

#[test]
fn test_put_bytes_with_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10); // Initialize with capacity 10
    bytes_mut.put_bytes(1, 10); // Should not panic, filling the capacity
    assert_eq!(bytes_mut.len(), 10); // Validate that the length is now 10
}

#[test]
fn test_put_bytes_with_reserve() {
    let mut bytes_mut = BytesMut::with_capacity(5); // Initialize with capacity 5
    bytes_mut.put_bytes(1, 5); // Should not panic, fills initial capacity
    assert_eq!(bytes_mut.len(), 5); // Validate length is 5
    bytes_mut.put_bytes(2, 5); // Should now reserve more, filling capacity up to 10
    assert_eq!(bytes_mut.len(), 10); // Validate length is now 10
}

#[test]
#[should_panic] // This should panic as the condition should fail
fn test_put_bytes_exceeding_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5); // Initialize with capacity 5
    bytes_mut.put_bytes(1, 6); // Would cause panic as it tries to write more than current capacity
}

#[test]
fn test_put_bytes_on_empty() {
    let mut bytes_mut = BytesMut::new(); // Create a new empty BytesMut
    bytes_mut.put_bytes(1, 0); // This should be valid and not panic
    assert_eq!(bytes_mut.len(), 0); // Length should remain 0
}

