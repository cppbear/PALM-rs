// Answer 0

#[test]
fn test_header_name_from_static_valid() {
    const VALID_HEADER: &'static str = "valid-header-123";
    let header_name = HeaderName::from_static(VALID_HEADER);
    assert_eq!(header_name.inner, Repr::Custom(Custom(ByteStr::from_static(VALID_HEADER))));
}

#[test]
#[should_panic]
fn test_header_name_from_static_empty() {
    HeaderName::from_static(""); // This should panic
}

#[test]
#[should_panic]
fn test_header_name_from_static_too_long() {
    let long_header: &str = &"a".repeat(super::MAX_HEADER_NAME_LEN + 1);
    HeaderName::from_static(long_header); // This should panic
}

#[test]
#[should_panic]
fn test_header_name_from_static_invalid_characters() {
    HeaderName::from_static("invalid{}header"); // This should panic
}

#[test]
#[should_panic]
fn test_header_name_from_static_uppercase() {
    HeaderName::from_static("UppercaseHeader"); // This should panic
}

