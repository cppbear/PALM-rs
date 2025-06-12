// Answer 0

#[test]
fn test_from_bytes_valid() {
    let valid_bytes = b"hello\xfa";
    let result = HeaderValue::from_bytes(valid_bytes).unwrap();
    assert_eq!(result.as_bytes(), valid_bytes);
}

#[test]
fn test_from_bytes_empty() {
    let empty_bytes: &[u8] = b"";
    let result = HeaderValue::from_bytes(empty_bytes).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_from_bytes_only_whitespace() {
    let whitespace_bytes = b"     "; // contains only spaces
    let result = HeaderValue::from_bytes(whitespace_bytes).unwrap();
    assert_eq!(result.as_bytes(), whitespace_bytes);
}

#[test]
fn test_from_bytes_invalid_control_character() {
    let invalid_bytes = b"\n";
    let result = HeaderValue::from_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_del_character() {
    let invalid_bytes = b"\x7f"; // DEL character
    let result = HeaderValue::from_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_contains_invalid_byte() {
    let mixed_bytes = b"valid\x00bytes"; // contains null byte
    let result = HeaderValue::from_bytes(mixed_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_boundary_valid() {
    let boundary_valid = b"\x20"; // space character (valid)
    let result = HeaderValue::from_bytes(boundary_valid).unwrap();
    assert_eq!(result.as_bytes(), boundary_valid);

    let boundary_valid_high = b"\xFF"; // 255 (valid)
    let result_high = HeaderValue::from_bytes(boundary_valid_high).unwrap();
    assert_eq!(result_high.as_bytes(), boundary_valid_high);
}

#[test]
fn test_from_bytes_boundary_invalid() {
    let boundary_invalid = b"\x1F"; // 31 (invalid)
    let result = HeaderValue::from_bytes(boundary_invalid);
    assert!(result.is_err());
}

