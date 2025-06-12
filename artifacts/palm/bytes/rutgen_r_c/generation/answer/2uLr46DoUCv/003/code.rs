// Answer 0

#[test]
fn test_truncate_with_len_less_than_current_length() {
    // Create a Bytes instance from a static slice
    let mut buf = Bytes::from_static(b"hello world");

    // Set the length to a value less than the current length
    let len_to_truncate = 5;

    // Truncate the buffer
    buf.truncate(len_to_truncate);

    // Ensure the length is now as expected
    assert_eq!(buf.len(), len_to_truncate);
    assert_eq!(buf.as_slice(), b"hello"[..]);
}

#[test]
fn test_truncate_does_nothing_if_len_is_equal() {
    // Create a Bytes instance from a static slice
    let mut buf = Bytes::from_static(b"hello");

    // Set the length to be equal to the current length
    let len_to_truncate = 5;

    // Truncate the buffer
    buf.truncate(len_to_truncate);

    // Ensure the length remains the same
    assert_eq!(buf.len(), len_to_truncate);
    assert_eq!(buf.as_slice(), b"hello"[..]);
}

#[test]
fn test_truncate_with_len_greater_than_current_length() {
    // Create a Bytes instance from a static slice
    let mut buf = Bytes::from_static(b"hello");

    // Set the length to a value greater than the current length
    let len_to_truncate = 10;

    // Truncate the buffer
    buf.truncate(len_to_truncate);

    // Ensure the length remains unchanged
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.as_slice(), b"hello"[..]);
}

