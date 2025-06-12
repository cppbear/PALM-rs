// Answer 0

#[test]
fn test_from_static_standard_header() {
    const STANDARD_HEADER: HeaderName = HeaderName::from_static("accept");
}

#[test]
fn test_from_static_custom_header_valid_length() {
    const CUSTOM_HEADER: HeaderName = HeaderName::from_static("custom-header-long");
}

#[test]
fn test_from_static_valid_length_at_max() {
    const VALID_HEADER_MAX: HeaderName = HeaderName::from_static("valid-header-name-long");
}

#[should_panic]
fn test_from_static_empty_string() {
    const EMPTY_STRING: HeaderName = HeaderName::from_static("");
}

#[should_panic]
fn test_from_static_invalid_length() {
    const INVALID_LENGTH_HEADER: HeaderName = HeaderName::from_static("invalid-{}-header");
}

#[should_panic]
fn test_from_static_invalid_uppercase() {
    const INVALID_UPPERCASE_HEADER: HeaderName = HeaderName::from_static("InvalidHeader");
}

