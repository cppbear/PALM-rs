// Answer 0

#[test]
fn test_from_static_valid_standard_header() {
    let header = HeaderName::from_static("accept");
    assert_eq!(header, HeaderName { inner: Repr::Standard(StandardHeader::Accept) });
}

#[test]
fn test_from_static_valid_custom_header() {
    let header = HeaderName::from_static("custom-header");
    let expected = HeaderName { inner: Repr::Custom(Custom(ByteStr::from_static("custom-header"))) };
    assert_eq!(header, expected);
}

#[test]
#[should_panic]
fn test_from_static_invalid_empty_string() {
    HeaderName::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_invalid_long_string() {
    let long_string = "a".repeat(super::MAX_HEADER_NAME_LEN + 1);
    HeaderName::from_static(&long_string);
}

#[test]
#[should_panic]
fn test_from_static_invalid_uppercase() {
    HeaderName::from_static("InvalidHeader");
}

#[test]
#[should_panic]
fn test_from_static_invalid_symbols() {
    HeaderName::from_static("content{}length");
}

#[test]
fn test_from_static_valid_boundary_case() {
    let valid_length_header = "a".repeat(super::MAX_HEADER_NAME_LEN);
    let header = HeaderName::from_static(&valid_length_header);
    let expected = HeaderName { inner: Repr::Custom(Custom(ByteStr::from_static(&valid_length_header))) };
    assert_eq!(header, expected);
}

#[test]
fn test_from_static_valid_sequence() {
    let valid_sequence_header = "valid-header-123";
    let header = HeaderName::from_static(valid_sequence_header);
    let expected = HeaderName { inner: Repr::Custom(Custom(ByteStr::from_static(valid_sequence_header))) };
    assert_eq!(header, expected);
}

