// Answer 0

#[test]
fn test_valid_static_header_lowercase() {
    const VALID_HEADER: &'static str = "valid-header";
    let result = HeaderName::from_static(VALID_HEADER);
    assert_eq!(result.as_str(), VALID_HEADER);
}

#[test]
fn test_valid_static_header_max_length() {
    const VALID_HEADER: &'static str = "a".repeat(super::MAX_HEADER_NAME_LEN - 1).as_str();
    let result = HeaderName::from_static(VALID_HEADER);
    assert_eq!(result.as_str(), VALID_HEADER);
}

#[test]
fn test_valid_static_standard_header() {
    const VALID_HEADER: &'static str = "accept";
    let result = HeaderName::from_static(VALID_HEADER);
    assert_eq!(result, ACCEPT);
}

#[test]
#[should_panic]
fn test_invalid_static_header_empty() {
    HeaderName::from_static("");
}

#[test]
#[should_panic]
fn test_invalid_static_header_too_long() {
    const LONG_HEADER: &'static str = "a".repeat(super::MAX_HEADER_NAME_LEN + 1).as_str();
    HeaderName::from_static(LONG_HEADER);
}

#[test]
#[should_panic]
fn test_invalid_static_header_invalid_character() {
    HeaderName::from_static("invalid{}header");
}

#[test]
#[should_panic]
fn test_invalid_static_header_uppercase() {
    HeaderName::from_static("InvalidHeader");
}

