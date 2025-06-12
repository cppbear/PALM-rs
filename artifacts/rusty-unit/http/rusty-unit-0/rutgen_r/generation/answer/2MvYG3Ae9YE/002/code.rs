// Answer 0

#[test]
fn test_from_lowercase_valid_header() {
    let hdr = from_lowercase(b"content-length").unwrap();
    assert_eq!(CONTENT_LENGTH, hdr);
}

#[test]
fn test_from_lowercase_invalid_header_uppercase() {
    assert!(from_lowercase(b"Content-Length").is_err());
}

#[test]
fn test_from_lowercase_invalid_header_non_ascii() {
    assert!(from_lowercase(b"invalid\x80header").is_err());
}

#[test]
fn test_from_lowercase_valid_custom_header_lower() {
    let hdr = from_lowercase(b"x-custom-header").unwrap();
    // Assuming we have an expected corresponding header for this custom header
    assert_eq!(CUSTOM_HEADER, hdr);
}

#[test]
#[should_panic]
fn test_from_lowercase_panic_on_invalid_utf8() {
    // This should panic if the implementation does not handle invalid UTF-8 encoding correctly
    let _ = from_lowercase(b"\xFF\xFE");
}

