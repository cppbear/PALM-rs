// Answer 0

#[test]
fn test_chunk_mut_capacity_equals_length() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    unsafe { bytes_mut.set_len(64) }; // Setting len to equal capacity

    let chunk = bytes_mut.chunk_mut(); // Should not panic
    assert_eq!(chunk.0.len(), 0); // Since we have reached capacity, spare should be empty
}

#[test]
fn test_chunk_mut_capacity_increases() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    unsafe { bytes_mut.set_len(64) }; // Setting len to equal capacity

    let chunk = bytes_mut.chunk_mut(); // This should trigger reserve, not panic
    assert!(bytes_mut.capacity() > 64); // Ensure capacity increased
    assert!(chunk.0.len() > 0); // sqare capacity should now have free space
}

#[test]
#[should_panic]
fn test_chunk_mut_panic_on_empty_capacity() {
    let mut bytes_mut = BytesMut::new(); // initial capacity is 0
    unsafe { bytes_mut.set_len(0) }; // Setting len to 0

    let _chunk = bytes_mut.chunk_mut(); // This should panic due to not enough capacity
}

