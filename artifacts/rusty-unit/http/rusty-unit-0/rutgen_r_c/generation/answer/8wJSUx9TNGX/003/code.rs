// Answer 0

#[test]
fn test_write_checked_valid_input() {
    let src: &[u8] = b"GET"; // Valid HTTP method
    let mut dst = [0u8; 3]; // Destination buffer with appropriate size

    let result = write_checked(src, &mut dst);

    assert_eq!(result, Ok(()));
    assert_eq!(dst, b"GET"); // Ensure the output matches the expected
}

#[test]
fn test_write_checked_invalid_input() {
    let src: &[u8] = b"\xFF"; // Invalid byte that does not correspond to valid characters
    let mut dst = [0u8; 1]; // Destination buffer

    let result = write_checked(src, &mut dst);

    assert!(result.is_err()); // Expect an error due to invalid input
}

#[test]
fn test_write_checked_empty_input() {
    let src: &[u8] = b""; // Empty input
    let mut dst: [u8; 0] = []; // Empty destination buffer

    let result = write_checked(src, &mut dst);

    assert_eq!(result, Ok(())); // Expect Ok(()) for empty input
}

