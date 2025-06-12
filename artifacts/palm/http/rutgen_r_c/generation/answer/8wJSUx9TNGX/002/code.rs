// Answer 0

#[test]
fn test_write_checked_valid_input() {
    let src: &[u8] = b"GET"; // Valid method
    let mut dst = [0u8; 3];
    assert_eq!(write_checked(src, &mut dst), Ok(()));
    assert_eq!(&dst, b"GET");
}

#[test]
fn test_write_checked_invalid_character() {
    let src: &[u8] = &[255]; // Invalid byte, > 127
    let mut dst = [0u8; 1];
    assert_eq!(write_checked(src, &mut dst), Err(InvalidMethod::new()));
}

#[test]
fn test_write_checked_empty_input() {
    let src: &[u8] = &[]; // Empty input
    let mut dst = [0u8; 0];
    assert_eq!(write_checked(src, &mut dst), Ok(())); // Should succeed
}

#[test]
fn test_write_checked_invalid_character_zero() {
    let src: &[u8] = &[0]; // Invalid because METHOD_CHARS[0] == 0
    let mut dst = [0u8; 1];
    assert_eq!(write_checked(src, &mut dst), Err(InvalidMethod::new()));
}

#[test]
fn test_write_checked_out_of_bounds_character() {
    let src: &[u8] = &[128]; // Out of bounds, invalid byte
    let mut dst = [0u8; 1];
    assert_eq!(write_checked(src, &mut dst), Err(InvalidMethod::new()));
}

#[test]
fn test_write_checked_valid_max_length() {
    let src: &[u8] = b"GET\r\n"; // Valid method with additional characters
    let mut dst = [0u8; 6];
    assert_eq!(write_checked(src, &mut dst), Ok(()));
    assert_eq!(&dst[0..4], b"GET\r"); // Only the valid characters should be written.
}

