// Answer 0

#[test]
fn test_parse_exact_https() {
    let input = b"https";
    let result = parse_exact(input);
    assert_eq!(result, Ok(Protocol::Https.into()));
}

#[test]
fn test_parse_exact_invalid_scheme() {
    let input = b"ftp"; // s does not match b"http" and should not be 'https'.
    let result = parse_exact(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_exact_scheme_too_long() {
    let long_input = b"this_scheme_is_too_long"; // Exceeds MAX_SCHEME_LEN.
    let result = parse_exact(long_input);
    assert!(result.is_err());
}

#[test]
fn test_parse_exact_invalid_character() {
    let invalid_input = b"invalid:scheme"; // Contains a forbidden ':' character.
    let result = parse_exact(invalid_input);
    assert!(result.is_err());
}

