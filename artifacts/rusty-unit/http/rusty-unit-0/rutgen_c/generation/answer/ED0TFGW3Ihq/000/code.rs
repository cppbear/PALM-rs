// Answer 0

#[test]
fn test_parse_exact_http() {
    let input = b"http";
    let result = Scheme2::<()>::parse_exact(input);
    let expected = Ok(Scheme2::Standard(Protocol::Http));
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_https() {
    let input = b"https";
    let result = Scheme2::<()>::parse_exact(input);
    let expected = Ok(Scheme2::Standard(Protocol::Https));
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_invalid_character() {
    let input = b"ht:p";
    let result = Scheme2::<()>::parse_exact(input);
    let expected = Err(InvalidUri(ErrorKind::InvalidScheme));
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_scheme_too_long() {
    let input = b"this_is_a_very_long_scheme_name_that_exceeds_the_max_length_allowed_by_the_parser";
    let result = Scheme2::<()>::parse_exact(input);
    let expected = Err(InvalidUri(ErrorKind::SchemeTooLong));
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_invalid_scheme_character() {
    let input = b"invalid@scheme";
    let result = Scheme2::<()>::parse_exact(input);
    let expected = Err(InvalidUri(ErrorKind::InvalidScheme));
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_empty_scheme() {
    let input = b"";
    let result = Scheme2::<()>::parse_exact(input);
    let expected = Err(InvalidUri(ErrorKind::SchemeMissing));
    assert_eq!(result, expected);
}

