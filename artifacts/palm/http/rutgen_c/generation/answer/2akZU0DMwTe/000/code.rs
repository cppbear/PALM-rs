// Answer 0

#[test]
fn test_parse_http_scheme() {
    let input = b"http://example.com";
    let result = Scheme2::parse(input);
    assert!(result.is_ok());
    match result.unwrap() {
        Scheme2::Standard(Protocol::Http) => (),
        _ => panic!("Expected Standard(Http), got {:?}", result),
    }
}

#[test]
fn test_parse_https_scheme() {
    let input = b"https://example.com";
    let result = Scheme2::parse(input);
    assert!(result.is_ok());
    match result.unwrap() {
        Scheme2::Standard(Protocol::Https) => (),
        _ => panic!("Expected Standard(Https), got {:?}", result),
    }
}

#[test]
fn test_parse_custom_scheme() {
    let input = b"ftp://example.com";
    let result = Scheme2::parse(input);
    assert!(result.is_ok());
    match result.unwrap() {
        Scheme2::Other(len) if len == 3 => (),
        _ => panic!("Expected Other with length 3, got {:?}", result),
    }
}

#[test]
fn test_parse_scheme_too_long() {
    let long_scheme = b"thisissuperlongthatsurelyexceedsmaximumlimits:valid";
    let result = Scheme2::parse(long_scheme);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, ErrorKind::SchemeTooLong);
}

#[test]
fn test_parse_invalid_scheme_character() {
    let input = b"ht@tp://example.com";
    let result = Scheme2::parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::None);
}

#[test]
fn test_parse_no_scheme() {
    let input = b"example.com";
    let result = Scheme2::parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::None);
}

