// Answer 0

#[test]
fn test_parse_exact_https() {
    let input = b"https";
    let expected = Ok(Scheme2::Standard(Protocol::Https));
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_invalid_scheme() {
    let input = b"ftp"; // not http or https
    let expected = Ok(Scheme2::Other(()));
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_scheme_too_long() {
    let input = b"this_scheme_is_too_long_to_be_valid"; // longer than 64 bytes
    let expected = Err(InvalidUri(ErrorKind::SchemeTooLong));
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_invalid_character() {
    let input = b"http:example"; // contains invalid character ':'
    let expected = Err(InvalidUri(ErrorKind::InvalidScheme));
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_valid_standard_http() {
    let input = b"http";
    let expected = Ok(Scheme2::Standard(Protocol::Http));
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, expected);
}

