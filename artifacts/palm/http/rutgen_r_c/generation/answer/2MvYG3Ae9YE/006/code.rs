// Answer 0

#[test]
fn test_from_lowercase_valid_standard_header() {
    struct TestHeader;

    impl TryFrom<&[u8]> for TestHeader {
        type Error = InvalidHeaderName;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            HeaderName::from_lowercase(value)?;
            Ok(TestHeader)
        }
    }

    let hdr = HeaderName::from_lowercase(b"accept").unwrap();
    assert_eq!(hdr, ACCEPT);
}

#[test]
fn test_from_lowercase_invalid_uppercase_header() {
    let result = HeaderName::from_lowercase(b"Accept");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_empty_input() {
    let result = HeaderName::from_lowercase(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_invalid_character() {
    let result = HeaderName::from_lowercase(b"content-length\xFF");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_valid_custom_header() {
    let hdr = HeaderName::from_lowercase(b"custom-header").unwrap();
    // Assuming Custom header handling is similar to Standard
    assert_eq!(hdr, Custom(ByteStr::from_static("custom-header")).into());
}

