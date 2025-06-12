// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    let input: &[u8] = b"Content-Type";
    let result = from_bytes(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_valid_custom_header_lower() {
    let input: &[u8] = b"x-custom-header";
    let result = from_bytes(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_valid_custom_header_upper() {
    let input: &[u8] = b"x-Custom-Header";
    let result = from_bytes(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_invalid_header_name_empty() {
    let input: &[u8] = b"";
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_header_name_invalid_char() {
    let input: &[u8] = b"Invalid/Header";
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_header_name_control_char() {
    let input: &[u8] = b"Header\x00Name";
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_header_name_non_utf8() {
    let input: &[u8] = &[0, 159, 146, 150]; // Invalid UTF-8 bytes
    let result = from_bytes(input);
    assert!(result.is_err());
}

