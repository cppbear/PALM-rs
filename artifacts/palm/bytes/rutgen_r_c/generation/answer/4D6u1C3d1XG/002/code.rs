// Answer 0

#[test]
fn test_split_to_beyond_length() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(5); // Set length to 5
    }
    
    // Attempt to split at a point greater than the current length to provoke a panic
    let result = std::panic::catch_unwind(|| {
        bytes_mut.split_to(6);
    });
    
    assert!(result.is_err(), "Expected panic when splitting at index greater than length");
}

#[test]
fn test_split_to_exact_length() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]); // Set length to 5

    let other = bytes_mut.split_to(5); // Should not panic

    assert_eq!(bytes_mut.len(), 0, "BytesMut should be empty after split");
    assert_eq!(other.len(), 5, "Returned BytesMut should contain all original elements");
}

#[test]
fn test_split_to_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]); // Set length to 5

    let other = bytes_mut.split_to(0); // Should not panic

    assert_eq!(bytes_mut.len(), 5, "BytesMut should retain its full length after splitting zero");
    assert_eq!(other.len(), 0, "Returned BytesMut should be empty after split at zero");
}

