// Answer 0

#[test]
fn test_as_str_standard() {
    let header_name = HeaderName::from_static("Content-Type");
    assert_eq!(header_name.as_str(), "Content-Type");
}

#[test]
fn test_as_str_custom() {
    let header_name = HeaderName::from_lowercase(b"custom-header").unwrap();
    assert_eq!(header_name.as_str(), "custom-header");
}

#[should_panic]
fn test_as_str_invalid() {
    let header_name = HeaderName::from_bytes(b"invalid\x80header").unwrap();
    // This should panic because the header name is invalid.
    let _ = header_name.as_str();
}

