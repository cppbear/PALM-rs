// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    let header_name = HeaderName::from_bytes(b"accept").unwrap();
    assert_eq!(header_name.as_str(), "accept");
}

#[test]
fn test_from_bytes_valid_custom_header() {
    let header_name = HeaderName::from_bytes(b"x-custom-header").unwrap();
    assert_eq!(header_name.as_str(), "x-custom-header");
}

#[test]
fn test_from_bytes_invalid_header_empty() {
    let result = HeaderName::from_bytes(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_header_contains_null() {
    let result = HeaderName::from_bytes(b"invalid\0header");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_custom_header_invalid_chars() {
    let result = HeaderName::from_bytes(b"invalid\xFFheader");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_overflow_header_name() {
    let long_header_name = [b'a'; SCRATCH_BUF_OVERFLOW]; // Overflow header name
    let result = HeaderName::from_bytes(&long_header_name);
    assert!(result.is_err());
}

