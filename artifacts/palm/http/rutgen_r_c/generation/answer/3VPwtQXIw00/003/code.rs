// Answer 0

#[test]
fn test_from_static_valid_standard_header() {
    let hdr = HeaderName::from_static("accept");
    assert_eq!(hdr.inner, Repr::Standard(StandardHeader::Accept));
}

#[test]
fn test_from_static_valid_custom_header() {
    let custom_header: &'static str = "custom-header";
    let hdr = HeaderName::from_static(custom_header);
    assert_eq!(hdr.inner, Repr::Custom(Custom(ByteStr::from_static(custom_header))));
}

#[test]
#[should_panic]
fn test_from_static_invalid_header_empty() {
    HeaderName::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_invalid_header_too_long() {
    let long_header: &str = "a".repeat(super::MAX_HEADER_NAME_LEN + 1).as_str();
    HeaderName::from_static(long_header);
}

#[test]
#[should_panic]
fn test_from_static_invalid_header_with_invalid_characters() {
    HeaderName::from_static("content{}length");
}

#[test]
#[should_panic]
fn test_from_static_invalid_header_with_uppercase() {
    HeaderName::from_static("FOOBAR");
}

#[test]
fn test_from_static_max_length_valid_header() {
    let max_length_header: &str = "a".repeat(super::MAX_HEADER_NAME_LEN);
    let hdr = HeaderName::from_static(max_length_header);
    assert_eq!(hdr.inner, Repr::Custom(Custom(ByteStr::from_static(max_length_header))));
}

