// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    let header = HeaderName::from_bytes(b"accept").unwrap();
    assert_eq!(header.as_str(), "accept");
}

#[test]
fn test_from_bytes_valid_custom_lowercase() {
    let header = HeaderName::from_bytes(b"custom-header").unwrap();
    assert_eq!(header.as_str(), "custom-header");
}

#[test]
fn test_from_bytes_invalid_character() {
    let result = HeaderName::from_bytes(b"invalid\xFFheader");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_empty() {
    let result = HeaderName::from_bytes(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_exceeding_max_header_name_length() {
    let long_header = vec![b'a'; super::MAX_HEADER_NAME_LEN + 1];
    let result = HeaderName::from_bytes(&long_header);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_valid_custom_uppercase() {
    let header = HeaderName::from_bytes(b"CUSTOM-HEADER").unwrap();
    assert_eq!(header.as_str(), "CUSTOM-HEADER");
}

#[test]
fn test_from_bytes_invalid_standard_header() {
    let header = HeaderName::from_bytes(b"unknown-header");
    assert!(header.is_err());
}

