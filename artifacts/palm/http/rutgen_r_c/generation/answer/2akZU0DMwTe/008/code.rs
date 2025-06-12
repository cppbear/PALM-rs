// Answer 0

#[test]
fn test_parse_http_scheme() {
    let input: &[u8] = b"htp://example.com";
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::Other(4)));
}

#[test]
fn test_parse_https_scheme() {
    let input: &[u8] = b"htts://example.com";
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::Other(5)));
}

#[test]
fn test_parse_scheme_with_long_length() {
    let input: &[u8] = b"verylongscheme:validpath//example.com";
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::Other(13)));
}

#[test]
fn test_parse_invalid_scheme_no_slash() {
    let input: &[u8] = b"invalidscheme:notvalidpath";
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::Other(14))); // 14 for the index of the colon
}

#[test]
fn test_parse_scheme_too_long() {
    let long_scheme: &[u8] = b"thisschemeiswaytoolongandshouldfail:valid//path";
    let result = Scheme2::<usize>::parse(long_scheme);
    assert!(result.is_err());
}

