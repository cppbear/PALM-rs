// Answer 0

#[test]
fn test_try_unsplit_with_empty_other() {
    let mut bytes_mut_a = BytesMut::with_capacity(10);
    let bytes_mut_b = BytesMut::new();
    
    let result = bytes_mut_a.try_unsplit(bytes_mut_b);
    
    assert!(result.is_ok());
}

#[test]
fn test_try_unsplit_contiguous_blocks() {
    let mut bytes_mut_a = BytesMut::with_capacity(10);
    let mut bytes_mut_b = BytesMut::with_capacity(10);
    
    unsafe {
        bytes_mut_a.set_len(5);
        bytes_mut_b.set_len(5);
    }
    
    // Manually simulate the condition of contiguous blocks
    bytes_mut_a.ptr = NonNull::new_unchecked(bytes_mut_a.ptr.as_ptr());
    bytes_mut_a.data = bytes_mut_b.data;
    
    let initial_len_a = bytes_mut_a.len();
    let initial_cap_a = bytes_mut_a.capacity();
    
    let result = bytes_mut_a.try_unsplit(bytes_mut_b);
    
    assert!(result.is_ok());
    assert_eq!(bytes_mut_a.len(), initial_len_a + bytes_mut_b.len());
    assert_eq!(bytes_mut_a.capacity(), initial_cap_a + bytes_mut_b.capacity());
}

#[test]
fn test_try_unsplit_non_contiguous_blocks() {
    let mut bytes_mut_a = BytesMut::with_capacity(10);
    let mut bytes_mut_b = BytesMut::with_capacity(10);

    let initial_len_a = bytes_mut_a.len();
    
    let result = bytes_mut_a.try_unsplit(bytes_mut_b);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().len(), initial_len_a);
}

