// Answer 0

#[test]
fn test_parse_exact_invalid_scheme_too_long() {
    let input = b"a".repeat(64).as_slice(); // Must be 64 in length
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::SchemeTooLong)));
}

#[test]
fn test_parse_exact_invalid_scheme_char_colon() {
    let input = b"invalid:scheme";
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

#[test]
fn test_parse_exact_invalid_scheme_char_zero() {
    let input = b"invalid\0scheme";
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

#[test]
fn test_parse_exact_valid_scheme_other() {
    let input = b"validscheme"; // No special restrictions except length
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Ok(Scheme2::Other(())));
}

#[test]
fn test_parse_exact_valid_scheme_http() {
    let input = b"http"; 
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Ok(Protocol::Http.into()));
}

#[test]
fn test_parse_exact_valid_scheme_https() {
    let input = b"https";
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Ok(Protocol::Https.into()));
}

