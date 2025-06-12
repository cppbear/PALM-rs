// Answer 0

#[test]
fn test_valid_standard_header() {
    let hdr = http::header::HeaderName::from_static("content-length");
    assert_eq!(hdr, http::header::CONTENT_LENGTH);
}

#[test]
fn test_valid_custom_header() {
    const CUSTOM_HEADER: &'static str = "custom-header";
    let a = http::header::HeaderName::from_lowercase(b"custom-header").unwrap();
    let b = http::header::HeaderName::from_static(CUSTOM_HEADER);
    assert_eq!(a, b);
}

#[should_panic]
#[test]
fn test_invalid_header_with_symbols() {
    http::header::HeaderName::from_static("content{}{}length");
}

#[should_panic]
#[test]
fn test_invalid_uppercase_header() {
    http::header::HeaderName::from_static("FOOBAR");
}

#[should_panic]
#[test]
fn test_empty_header() {
    http::header::HeaderName::from_static("");
}

#[should_panic]
#[test]
fn test_too_long_header() {
    let long_header = "a".repeat(http::header::MAX_HEADER_NAME_LEN + 1);
    http::header::HeaderName::from_static(&long_header);
}

