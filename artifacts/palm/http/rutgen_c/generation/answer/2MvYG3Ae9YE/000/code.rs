// Answer 0

#[test]
fn test_from_lowercase_valid() {
    let hdr = HeaderName::from_lowercase(b"accept").unwrap();
    assert_eq!(hdr, HeaderName { inner: Repr::Standard(StandardHeader::Accept) });
}

#[test]
fn test_from_lowercase_invalid() {
    assert!(HeaderName::from_lowercase(b"Content-Type").is_err());
}

#[test]
fn test_from_lowercase_empty() {
    assert!(HeaderName::from_lowercase(b"").is_err());
}

#[test]
fn test_from_lowercase_overflow() {
    let long_input = b"header-with-very-long-byte-sequence";
    assert!(HeaderName::from_lowercase(long_input).is_err());
}

#[test]
fn test_from_lowercase_invalid_chars() {
    assert!(HeaderName::from_lowercase(b"invalid\xFFheader").is_err());
}

