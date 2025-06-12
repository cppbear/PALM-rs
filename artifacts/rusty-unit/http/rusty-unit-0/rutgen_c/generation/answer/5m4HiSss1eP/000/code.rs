// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    let header = HeaderName::from_bytes(b"accept").unwrap();
    assert_eq!(header, HeaderName { inner: Repr::Standard(StandardHeader::Accept) });
}

#[test]
fn test_from_bytes_valid_custom_header() {
    let header = HeaderName::from_bytes(b"X-Custom-Header").unwrap();
    let expected = Custom(ByteStr::from_static("X-Custom-Header"));
    assert_eq!(header, expected.into());
}

#[test]
fn test_from_bytes_invalid_header_name_empty() {
    let result = HeaderName::from_bytes(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_header_name_overflow() {
    let long_header = b"X-A".repeat(SCRATCH_BUF_OVERFLOW); // This exceeds the allowable limit.
    let result = HeaderName::from_bytes(&long_header);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_header_name_invalid_char() {
    let result = HeaderName::from_bytes(b"invalid\xFFheader");
    assert!(result.is_err());
}

