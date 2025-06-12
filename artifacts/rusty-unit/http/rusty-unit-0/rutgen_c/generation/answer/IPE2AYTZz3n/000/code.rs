// Answer 0

#[test]
fn test_header_name_as_str_standard() {
    let header_name = HeaderName::from_static("Test-Header");
    assert_eq!(header_name.as_str(), "Test-Header");
}

#[test]
fn test_header_name_as_str_custom() {
    let custom_header_name = HeaderName::from_bytes(b"Custom-Header").unwrap();
    assert_eq!(custom_header_name.as_str(), "Custom-Header");
}

#[test]
fn test_header_name_as_str_lowercase() {
    let lowercase_header_name = HeaderName::from_lowercase(b"lowercase-header").unwrap();
    assert_eq!(lowercase_header_name.as_str(), "lowercase-header");
}

