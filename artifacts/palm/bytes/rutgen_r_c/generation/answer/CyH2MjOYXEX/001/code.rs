// Answer 0

#[test]
fn test_remaining_mut_non_empty() {
    let bytes_mut = BytesMut::new(); // Initialize a new BytesMut
    unsafe {
        bytes_mut.set_len(5); // Set length to 5
    }
    assert_eq!(bytes_mut.remaining_mut(), usize::MAX - 5);
}

#[test]
fn test_remaining_mut_empty() {
    let bytes_mut = BytesMut::new(); // Initialize a new BytesMut
    assert_eq!(bytes_mut.remaining_mut(), usize::MAX);
}

#[test]
fn test_remaining_mut_max_length() {
    let bytes_mut = BytesMut::new(); // Initialize a new BytesMut
    unsafe {
        bytes_mut.set_len(usize::MAX);
    }
    assert_eq!(bytes_mut.remaining_mut(), 0); // Expected return when length is at maximum
}

#[test]
#[should_panic]
fn test_remaining_mut_overflow() {
    let bytes_mut = BytesMut::new(); // Initialize a new BytesMut
    unsafe {
        bytes_mut.set_len(usize::MAX - 1); // Set length to near max
    }
    let _ = bytes_mut.remaining_mut(); // Checking this may cause underflow panic
}

