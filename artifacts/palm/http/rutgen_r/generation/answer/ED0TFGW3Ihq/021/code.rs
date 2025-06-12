// Answer 0

#[test]
fn test_parse_exact_http() {
    let input: &[u8] = b"http";
    let result = parse_exact(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        assert_eq!(scheme, Protocol::Http.into());
    }
}

#[test]
fn test_parse_exact_https() {
    let input: &[u8] = b"https";
    let result = parse_exact(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        assert_eq!(scheme, Protocol::Https.into());
    }
}

#[test]
fn test_parse_exact_other_scheme() {
    let input: &[u8] = b"ftp";
    let result = parse_exact(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        assert_eq!(scheme, Scheme2::Other(()));
    }
}

#[test]
fn test_parse_exact_invalid_scheme_too_long() {
    let input: &[u8] = b"this_scheme_is_way_too_long";
    let result = parse_exact(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_exact_invalid_scheme_char() {
    let input: &[u8] = b"invalid:scheme";
    let result = parse_exact(input);
    assert!(result.is_err());
}

