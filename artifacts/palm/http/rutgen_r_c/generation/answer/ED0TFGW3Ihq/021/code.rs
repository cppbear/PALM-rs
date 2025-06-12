// Answer 0

#[test]
fn test_parse_exact_http_scheme() {
    let input = b"http";
    let expected = Ok(Scheme2::Standard(Protocol::Http));
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_https_scheme() {
    let input = b"https";
    let expected = Ok(Scheme2::Standard(Protocol::Https));
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, expected);
}

#[test]
fn test_parse_exact_invalid_scheme() {
    let input = b"ftp";
    let result = Scheme2::<()>::parse_exact(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_exact_invalid_characters() {
    let input = b"http:example.com";
    let result = Scheme2::<()>::parse_exact(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_exact_too_long_scheme() {
    let input = b"this_scheme_is_way_too_long_and_exceeds_the_maximum_length_allowed";
    let result = Scheme2::<()>::parse_exact(input);
    assert!(result.is_err());
}

