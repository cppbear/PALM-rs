// Answer 0

#[test]
fn test_from_lowercase_valid() {
    let hdr = HeaderName::from_lowercase(b"content-length").unwrap();
    assert_eq!(CONTENT_LENGTH, hdr);
}

#[test]
fn test_from_lowercase_invalid_uppercase() {
    assert!(HeaderName::from_lowercase(b"Content-Length").is_err());
}

#[test]
fn test_from_lowercase_empty() {
    assert!(HeaderName::from_lowercase(b"").is_err());
}

#[test]
fn test_from_lowercase_invalid_control_char() {
    assert!(HeaderName::from_lowercase(b"\n").is_err());
}

#[test]
fn test_from_lowercase_invalid_non_ascii() {
    assert!(HeaderName::from_lowercase(b"content-length\xFF").is_err());
}

