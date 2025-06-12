// Answer 0

#[test]
fn test_copy_to_bytes() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);

    // Obtain bytes from the copy_to_bytes method
    let bytes = bytes_mut.copy_to_bytes(3);

    // Validate the length of the returned Bytes
    assert_eq!(bytes.len(), 3);

    // Validate the original BytesMut's length
    assert_eq!(bytes_mut.len(), 2); // Should have 2 remaining bytes after copy

    // Validate the contents of Bytes
    assert_eq!(bytes.chunk(), &[1, 2, 3]);

    // Validate the contents of the remaining BytesMut
    // Since original was [1, 2, 3, 4, 5], after copying [1, 2, 3],
    // BytesMut should contain [4, 5]
    assert_eq!(bytes_mut.as_slice(), &[4, 5]);
}

#[test]
#[should_panic(expected = "split_to out of bounds: 5 <= 5")]
fn test_copy_to_bytes_out_of_bounds() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.extend_from_slice(&[1, 2, 3]);

    // This should panic because the length exceeds the available bytes
    let _ = bytes_mut.copy_to_bytes(5);
}

