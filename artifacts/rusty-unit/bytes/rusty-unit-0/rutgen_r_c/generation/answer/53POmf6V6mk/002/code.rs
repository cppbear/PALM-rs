// Answer 0

#[test]
fn test_extend_from_slice_with_inadequate_capacity() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(5, 0); // Fill the buffer to capacity

    // Attempt to extend beyond current capacity
    buf.extend_from_slice(b"abcdef"); // This should trigger an error since buffer cannot accommodate 6 bytes

    assert_eq!(buf.len(), 5); // Length should remain the same since the operation should panic/abort
}

#[test]
fn test_extend_from_slice_with_zero_length() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b""); // Extending with an empty slice

    assert_eq!(buf.len(), 0); // Length should be 0 since nothing is added
}

#[test]
fn test_extend_from_slice_exact_fit() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(5, 0);

    // Correctly extending the buffer with exactly 5 bytes
    buf.extend_from_slice(b"abcde");

    assert_eq!(buf.len(), 5);
    assert_eq!(&buf.as_slice()[..], b"abcde"); // Ensure the contents match
}

#[test]
fn test_extend_from_slice_over_maximum_capacity() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(10, 0);

    // Extend with more than max capacity
    buf.extend_from_slice(&[1; 20]); // Should resize and accommodate new data

    assert_eq!(buf.len(), 20); // Length should increase after extension
}

#[test]
#[should_panic(expected = "dst.len() >= cnt is false")]
fn test_extend_from_slice_panic_case() {
    let mut buf = BytesMut::with_capacity(4); // Buffer capacity of 4
    buf.resize(4, 0); // Fill the buffer to capacity

    // Attempt extending with a slice too large, should panic
    buf.extend_from_slice(b"abcdef"); // More bytes than the current capacity
}

