// Answer 0

#[test]
fn test_header_name_from_static_valid_standard() {
    let hdr = HeaderName::from_static("accept");
    assert_eq!(hdr, HEADER_NAME::ACCEPT);
}

#[test]
fn test_header_name_from_static_valid_custom() {
    let custom_header: &'static str = "custom-header";
    let hdr = HeaderName::from_static(custom_header);
    assert_eq!(hdr.as_str(), "custom-header");
}

#[should_panic(expected = "Invalid header name")]
#[test]
fn test_header_name_from_static_invalid_empty() {
    HeaderName::from_static("");
}

#[should_panic(expected = "Invalid header name")]
#[test]
fn test_header_name_from_static_invalid_uppercase() {
    HeaderName::from_static("InvalidHeader");
}

#[should_panic(expected = "Invalid header name")]
#[test]
fn test_header_name_from_static_invalid_symbols() {
    HeaderName::from_static("content{}length");
}

#[test]
fn test_header_name_from_static_valid_boundary_length() {
    let valid_length_header = "a".repeat(super::MAX_HEADER_NAME_LEN);
    let hdr = HeaderName::from_static(&valid_length_header);
    assert_eq!(hdr.as_str(), valid_length_header);
}

#[should_panic(expected = "Invalid header name")]
#[test]
fn test_header_name_from_static_invalid_boundary_length_exceeded() {
    let invalid_length_header = "a".repeat(super::MAX_HEADER_NAME_LEN + 1);
    HeaderName::from_static(&invalid_length_header);
}

