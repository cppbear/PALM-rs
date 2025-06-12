// Answer 0

#[test]
fn test_parse_http() {
    let input = b"http://example.com";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::Http);
}

#[test]
fn test_parse_https() {
    let input = b"https://example.com";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::Https);
}

#[test]
fn test_parse_other_scheme() {
    let input = b"ftp://example.com";
    let result = parse(input);
    assert!(result.is_ok());
    if let Scheme2::Other(len) = result.unwrap() {
        assert!(len > 0);
    } else {
        panic!("Expected Other variant");
    }
}

#[test]
fn test_parse_scheme_too_long() {
    let long_scheme = b"http://reallylongscheme:80/";
    let result = parse(long_scheme);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind(), ErrorKind::SchemeTooLong);
}

#[test]
fn test_parse_invalid_scheme() {
    let input = b"invalidscheme:example.com";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::None);
}

#[test]
fn test_parse_empty() {
    let input: &[u8] = b"";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::None);
}

#[test]
fn test_parse_short_http() {
    let input = b"http:/example.com";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::None);
}

#[test]
fn test_parse_invalid_character() {
    let input = b"ht:p://example.com";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Scheme2::None);
}

