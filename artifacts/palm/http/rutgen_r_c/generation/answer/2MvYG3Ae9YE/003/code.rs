// Answer 0

#[test]
fn test_from_lowercase_valid() {
    let hdr = HeaderName::from_lowercase(b"content-length").unwrap();
    assert_eq!(CONTENT_LENGTH, hdr);
}

#[test]
fn test_from_lowercase_uppercase() {
    let result = HeaderName::from_lowercase(b"Content-Length");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_empty() {
    let result = HeaderName::from_lowercase(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_invalid_character() {
    let result = HeaderName::from_lowercase(b"content-length\xFF");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_valid_custom() {
    // Assuming Custom can be instantiated, using a valid lowercase string
    let hdr = HeaderName::from_lowercase(b"valid-header").unwrap();
    let custom = Custom(ByteStr::from_static("valid-header"));
    assert_eq!(hdr.inner, Repr::Custom(custom));
}

