// Answer 0

#[test]
fn test_write_checked_valid_input() {
    let src: &[u8] = b"GET";
    let mut dst = [0u8; 3];
    let result = write_checked(src, &mut dst);
    assert!(result.is_ok());
    assert_eq!(dst, [b'G', b'E', b'T']);
}

#[test]
fn test_write_checked_invalid_input() {
    let src: &[u8] = b"\xFF"; // Invalid byte (not in METHOD_CHARS)
    let mut dst = [0u8; 1];
    let result = write_checked(src, &mut dst);
    assert!(result.is_err());
}

#[test]
fn test_write_checked_empty_input() {
    let src: &[u8] = b""; // Empty input
    let mut dst = [0u8; 0];
    let result = write_checked(src, &mut dst);
    assert!(result.is_ok());
}

#[test]
fn test_write_checked_partial_valid_partial_invalid() {
    let src: &[u8] = b"GET\xFF"; // Valid followed by an invalid byte
    let mut dst = [0u8; 4];
    let result = write_checked(src, &mut dst);
    assert!(result.is_err());
}

