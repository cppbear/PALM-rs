// Answer 0

#[test]
fn test_from_str_valid_header_name() {
    let result = HeaderName::from_str("Accept");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_valid_custom_header_name() {
    let result = HeaderName::from_str("X-Custom-Header");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_empty_string() {
    let result = HeaderName::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_header_name() {
    let result = HeaderName::from_str("Invalid@Header");
    assert!(result.is_err());
}

#[test]
fn test_from_str_valid_lowercase_header_name() {
    let result = HeaderName::from_str("content-type");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_valid_ascii_header_name() {
    let result = HeaderName::from_str("Accept-Encoding");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_valid_uppercase_header_name() {
    let result = HeaderName::from_str("ACCEPT-CHARSET");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_valid_standard_header_name() {
    let result = HeaderName::from_str("Age");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_header_with_spaces() {
    let result = HeaderName::from_str("Invalid Header");
    assert!(result.is_err());
}

#[test]
fn test_from_str_header_with_special_characters() {
    let result = HeaderName::from_str("Header-!@#");
    assert!(result.is_err());
}

