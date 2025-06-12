// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    let input: &[u8] = b"Content-Type";
    let result = from_bytes(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_valid_custom_header_lower() {
    let input: &[u8] = b"custom-header";
    let result = from_bytes(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_valid_custom_header_upper() {
    let input: &[u8] = b"CUSTOM-HEADER";
    let result = from_bytes(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_invalid_header_name() {
    let input: &[u8] = b"invalid\xFFheader";
    let result = from_bytes(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "InvalidHeaderName");
}

#[test]
fn test_from_bytes_empty_input() {
    let input: &[u8] = b"";
    let result = from_bytes(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "InvalidHeaderName");
}

#[test]
fn test_from_bytes_contains_zero_byte() {
    let input: &[u8] = b"header\x00name";
    let result = from_bytes(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "InvalidHeaderName");
}

