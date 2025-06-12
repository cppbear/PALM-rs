// Answer 0

#[test]
fn test_chunk_mut_with_non_full_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(128);
    // Set length less than capacity to avoid triggering the reserve
    unsafe {
        bytes_mut.set_len(64);
    }
    let result: &mut UninitSlice = bytes_mut.chunk_mut();
    assert_eq!(result.0.len(), bytes_mut.capacity() - bytes_mut.len());
}

#[test]
fn test_chunk_mut_with_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(128);
    // Set length to equal capacity to trigger the reserve
    unsafe {
        bytes_mut.set_len(128);
    }
    let result: &mut UninitSlice = bytes_mut.chunk_mut();
    // After calling chunk_mut, we expect that there are spare capacity bytes available
    assert!(bytes_mut.capacity() > bytes_mut.len());
    assert_eq!(result.0.len(), bytes_mut.capacity() - bytes_mut.len());
}

