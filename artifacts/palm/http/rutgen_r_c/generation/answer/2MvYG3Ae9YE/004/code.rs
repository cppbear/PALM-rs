// Answer 0

#[test]
fn test_from_lowercase_valid() {
    let hdr = HeaderName::from_lowercase(b"content-length").unwrap();
    assert_eq!(hdr, HeaderName { inner: Repr::Standard(StandardHeader::ContentLength) });
}

#[test]
fn test_from_lowercase_uppercase_error() {
    assert!(HeaderName::from_lowercase(b"Content-Length").is_err());
}

#[test]
fn test_from_lowercase_invalid_characters() {
    assert!(HeaderName::from_lowercase(b"invalid\xFFheader").is_err());
}

#[test]
fn test_from_lowercase_empty() {
    assert!(HeaderName::from_lowercase(b"").is_err());
}

#[test]
fn test_from_lowercase_mixed_case_error() {
    assert!(HeaderName::from_lowercase(b"Content-LengTH").is_err());
}

#[test]
fn test_from_lowercase_valid_custom() {
    let hdr = HeaderName::from_lowercase(b"my-custom-header").unwrap();
    assert_eq!(hdr.inner, Repr::Custom(Custom(ByteStr::from_static("my-custom-header"))));
}

#[test]
fn test_from_lowercase_non_utf8_character_error() {
    let input = b"\x80invalid-header"; // Invalid UTF-8 byte
    assert!(HeaderName::from_lowercase(input).is_err());
}

