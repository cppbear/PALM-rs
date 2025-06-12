// Answer 0

#[test]
fn test_chunk_mut_increases_capacity() {
    // Initialize a BytesMut with a capacity of 10
    let mut bytes_mut = BytesMut::with_capacity(10);
    
    // Set the length to the current capacity to trigger reserve
    unsafe { bytes_mut.set_len(10) };
    
    let chunk = bytes_mut.chunk_mut();
    assert_eq!(bytes_mut.capacity(), 10); // Capacity should remain 10
    assert_eq!(bytes_mut.len(), 10);      // Length should still be 10
    
    // Check if the chunk is correctly pointing to spare capacity
    let spare_capacity = bytes_mut.spare_capacity_mut();
    assert_eq!(spare_capacity.len(), 0); // Initially should have 0 spare capacity
    
    // Call chunk_mut which should reserve more space
    let chunk_after = bytes_mut.chunk_mut();
    assert!(bytes_mut.capacity() > 10); // Capacity should increase after calling chunk_mut
}

#[test]
fn test_chunk_mut_returns_correct_slice() {
    // Initialize a BytesMut with a capacity of 64
    let mut bytes_mut = BytesMut::with_capacity(64);
    
    // Set the length to 0 to ensure there is spare capacity
    unsafe { bytes_mut.set_len(0) };
    
    let chunk = bytes_mut.chunk_mut();
    let spare = bytes_mut.spare_capacity_mut();
    
    // Ensure that chunk_mut returns a slice that can be written to
    assert_eq!(chunk.0.len(), spare.len()); // Ensure the sizes match
    
    // Write to the chunk
    chunk.0[0].write(1u8); // safe write to allocated space
    assert_eq!(spare[0].assume_init(), 1u8); // Verify the value written
}

#[test]
fn test_chunk_mut_when_empty() {
    // Create a new BytesMut which has len = 0 and capacity = 0
    let mut bytes_mut = BytesMut::new();
    
    let chunk = bytes_mut.chunk_mut();
    
    // Assert that it reserves as expected and returns a valid slice
    assert!(bytes_mut.capacity() > 0); // Capacity should increase
    assert_eq!(chunk.0.len(), bytes_mut.capacity()); // Length of chunk should match new capacity
}

