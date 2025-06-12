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
fn test_from_bytes_invalid_empty() {
    let result = HeaderName::from_bytes(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_non_ascii() {
    let result = HeaderName::from_bytes(b"\xFF");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_with_zero_byte() {
    let result = HeaderName::from_bytes(b"valid\x00header");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_overflow_length() {
    let long_input = [b'a'; SCRATCH_BUF_OVERFLOW + 1];
    let result = HeaderName::from_bytes(&long_input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_exactly_scratch_buf_size() {
    let valid_input = [b'a'; SCRATCH_BUF_SIZE];
    let result = HeaderName::from_bytes(&valid_input);
    assert!(result.is_ok());
}

