// Answer 0

#[test]
fn test_parse_exact_https() {
    let result = Scheme2::<()>::parse_exact(b"https");
    assert_eq!(result, Ok(Scheme2::Standard(Protocol::Https)));
}

#[test]
fn test_parse_exact_invalid_scheme_length() {
    let long_scheme = b"abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh";
    let result = Scheme2::<()>::parse_exact(long_scheme);
    assert_eq!(result, Err(InvalidUri(ErrorKind::SchemeTooLong)));
}

#[test]
fn test_parse_exact_contains_colon() {
    let scheme = b"invalid:s";
    let result = Scheme2::<()>::parse_exact(scheme);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

#[test]
fn test_parse_exact_contains_invalid_character() {
    let scheme = b"invalid<";
    let result = Scheme2::<()>::parse_exact(scheme);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

