// Answer 0

#[test]
fn test_from_static_valid_custom_header() {
    // Valid header name that conforms to lowercase and symbol constraints
    const VALID_HEADER: &'static str = "custom-header";
    let hdr = HeaderName::from_static(VALID_HEADER);
    assert_eq!(hdr.inner, Repr::Custom(Custom(ByteStr::from_static(VALID_HEADER))));
}

#[test]
#[should_panic]
fn test_from_static_empty_header() {
    // Empty header name
    HeaderName::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_too_long_header() {
    // Header name that exceeds MAX_HEADER_NAME_LEN
    const TOO_LONG_HEADER: &'static str = "a".repeat(100); // Assuming MAX_HEADER_NAME_LEN < 100
    HeaderName::from_static(TOO_LONG_HEADER.as_str());
}

#[test]
#[should_panic]
fn test_from_static_invalid_symbols() {
    // Header name that contains invalid symbols
    HeaderName::from_static("invalid{}header");
}

#[test]
#[should_panic]
fn test_from_static_uppercase_header() {
    // Header name that contains uppercase characters
    HeaderName::from_static("UppercaseHeader");
}

