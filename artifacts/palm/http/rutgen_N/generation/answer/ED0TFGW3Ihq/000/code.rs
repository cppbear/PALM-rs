// Answer 0

#[test]
fn test_parse_exact_http() {
    let input = b"http";
    let result = parse_exact(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::Http(()));
}

#[test]
fn test_parse_exact_https() {
    let input = b"https";
    let result = parse_exact(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::Https(()));
}

#[test]
fn test_parse_exact_invalid_scheme_too_long() {
    let input = b"invalid_scheme_which_is_too_long";
    let result = parse_exact(input);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), InvalidUri::SchemeTooLong));
}

#[test]
fn test_parse_exact_invalid_scheme_char() {
    let input = b"http://";
    let result = parse_exact(input);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), InvalidUri::InvalidScheme));
}

#[test]
fn test_parse_exact_valid_other_scheme() {
    let input = b"customscheme";
    let result = parse_exact(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::Other(()));
}

#[test]
fn test_parse_exact_invalid_scheme_empty() {
    let input = b"";
    let result = parse_exact(input);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), InvalidUri::InvalidScheme));
}

