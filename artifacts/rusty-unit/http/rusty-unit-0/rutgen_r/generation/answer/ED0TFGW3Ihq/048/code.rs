// Answer 0

#[test]
fn test_parse_exact_scheme_too_long() {
    const MAX_SCHEME_LEN: usize = 10; // Assuming MAX_SCHEME_LEN is defined as 10 for the context
    let long_scheme: &[u8] = b"this_scheme_is_too_long"; // Length exceeds MAX_SCHEME_LEN

    let result = parse_exact(long_scheme);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::SchemeTooLong);
}

#[test]
fn test_parse_exact_invalid_scheme_character() {
    let invalid_scheme: &[u8] = b"invalid:scheme"; // Contains a ':'
    
    let result = parse_exact(invalid_scheme);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidScheme);
}

#[test]
fn test_parse_exact_non_utf8_scheme() {
    let non_utf8_scheme: &[u8] = &[255]; // A byte that is not a valid UTF-8 code point

    let result = parse_exact(non_utf8_scheme);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidScheme);
}

#[test]
fn test_parse_exact_valid_other_scheme() {
    let valid_scheme: &[u8] = b"ftp"; // A valid scheme that's not HTTP or HTTPS

    let result = parse_exact(valid_scheme);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        assert_matches!(scheme, Scheme2::Other(()));
    }
}

