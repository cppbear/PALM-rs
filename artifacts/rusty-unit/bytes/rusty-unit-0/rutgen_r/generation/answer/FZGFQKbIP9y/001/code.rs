// Answer 0

#[test]
fn test_set_len_equal_to_capacity() {
    use bytes::BytesMut;

    // Create a BytesMut with a specific capacity.
    let mut b = BytesMut::with_capacity(11);

    // Initialize the buffer with valid data.
    b.extend_from_slice(&b"hello world"[..]);

    // Set length equal to capacity (11).
    unsafe {
        b.set_len(11);
    }
    
    // Assert that the buffer length is correct.
    assert_eq!(b.len(), 11);
    assert_eq!(&b[..], b"hello world");
}

#[test]
fn test_set_len_smaller_than_capacity() {
    use bytes::BytesMut;

    // Create a BytesMut with a specific capacity.
    let mut b = BytesMut::with_capacity(11);

    // Initialize the buffer with valid data.
    b.extend_from_slice(&b"hello world"[..]);

    // Set length to a smaller value (5).
    unsafe {
        b.set_len(5);
    }
    
    // Assert that the buffer length is correct.
    assert_eq!(b.len(), 5);
    assert_eq!(&b[..], b"hello");
}

#[test]
#[should_panic]
fn test_set_len_exceeding_capacity() {
    use bytes::BytesMut;

    // Create a BytesMut with a specific capacity.
    let mut b = BytesMut::with_capacity(10);

    // Try to set length greater than capacity (11).
    unsafe {
        b.set_len(11);
    }
}

#[test]
fn test_set_len_zero_length() {
    use bytes::BytesMut;

    // Create a BytesMut with a specific capacity.
    let mut b = BytesMut::with_capacity(11);

    // Initialize the buffer with valid data.
    b.extend_from_slice(&b"hello world"[..]);

    // Set length to zero.
    unsafe {
        b.set_len(0);
    }
    
    // Assert that the buffer length is correct.
    assert_eq!(b.len(), 0);
    assert_eq!(&b[..], b"");
}

