// Answer 0

#[test]
fn test_from_bytes_valid() {
    let valid_bytes = b"ValidHeaderValue";
    let result = HeaderValue::from_bytes(valid_bytes);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.as_bytes(), valid_bytes);
}

#[test]
fn test_from_bytes_valid_with_control_char() {
    let valid_bytes = b"Valid\xfaHeaderValue"; // 0xfa is allowed
    let result = HeaderValue::from_bytes(valid_bytes);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.as_bytes(), valid_bytes);
}

#[test]
fn test_from_bytes_invalid_with_control_character() {
    let invalid_bytes = b"\n"; // Newline is not allowed
    let result = HeaderValue::from_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_with_del_character() {
    let invalid_bytes = b"ValidHeaderValue\x7f"; // 0x7f (DEL) is not allowed
    let result = HeaderValue::from_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_empty() {
    let empty_bytes: &[u8] = b""; // Should be valid as no invalid bytes are present
    let result = HeaderValue::from_bytes(empty_bytes);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.as_bytes(), empty_bytes);
}

#[test]
fn test_from_bytes_boundary_upper() {
    let upper_boundary_bytes = b"ValidHeaderValue\xff"; // 0xff is allowed
    let result = HeaderValue::from_bytes(upper_boundary_bytes);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.as_bytes(), upper_boundary_bytes);
}

#[test]
fn test_from_bytes_boundary_lower() {
    let lower_boundary_bytes = b"ValidHeaderValue "; // Space (0x20) is allowed
    let result = HeaderValue::from_bytes(lower_boundary_bytes);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.as_bytes(), lower_boundary_bytes);
}

