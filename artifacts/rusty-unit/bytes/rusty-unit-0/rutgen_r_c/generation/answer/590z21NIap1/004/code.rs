// Answer 0

#[test]
fn test_advance_unchecked_zero_count() {
    let mut buffer = unsafe { BytesMut::with_capacity(10) }; // Initialize with some capacity
    unsafe {
        buffer.advance_unchecked(0); // Should return early
    }
    assert_eq!(buffer.len(), 0); // Ensure length remains unchanged
    assert_eq!(buffer.capacity(), 10); // Ensure capacity remains unchanged
}

#[test]
fn test_advance_unchecked_with_capacity() {
    let mut buffer = unsafe { BytesMut::with_capacity(5) }; // Initialize with capacity 5
    unsafe {
        buffer.resize(5, 0); // Resize to use full capacity
        buffer.advance_unchecked(5); // Advance by full capacity
    }
    assert_eq!(buffer.len(), 0); // All length should be consumed
    assert_eq!(buffer.capacity(), 5); // Capacity should remain unchanged
}

#[test]
fn test_advance_unchecked_not_vec_kind() {
    // Since we can't directly set the kind, we will create a `BytesMut` in a non-vec state and ensure the call does not panic.
    // Assuming BytesMut::new() creates a state where kind is not KIND_VEC.
    let mut buffer = BytesMut::new();
    buffer.reserve(10); // Reserve some capacity
    // Advance with count < cap, ensuring we are in a non-kind vec state.
    unsafe {
        buffer.advance_unchecked(1); // This should succeed as `kind` is not KIND_VEC
    }
    assert_eq!(buffer.len(), 0); // Length should remain unchanged
    assert_eq!(buffer.capacity(), 10); // Capacity should remain unchanged
}

