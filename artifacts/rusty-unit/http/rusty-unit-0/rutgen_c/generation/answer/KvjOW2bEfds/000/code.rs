// Answer 0

#[test]
fn test_parse_full_http() {
    let input = Bytes::from_static(b"http://example.com/path?query#fragment");
    let result = parse_full(input).unwrap();
    assert_eq!(result.scheme, Scheme2::Standard(Protocol::Http));
    assert_eq!(result.authority.as_str(), "example.com");
    assert_eq!(result.path_and_query.as_str(), "/path");
}

#[test]
fn test_parse_full_https() {
    let input = Bytes::from_static(b"https://example.com/path?query#fragment");
    let result = parse_full(input).unwrap();
    assert_eq!(result.scheme, Scheme2::Standard(Protocol::Https));
    assert_eq!(result.authority.as_str(), "example.com");
    assert_eq!(result.path_and_query.as_str(), "/path");
}

#[test]
fn test_parse_full_with_no_authority() {
    let input = Bytes::from_static(b"example/path?query#fragment");
    let result = parse_full(input).unwrap();
    assert!(result.scheme.is_none());
    assert_eq!(result.authority.as_str(), "example");
    assert_eq!(result.path_and_query.as_str(), "/path");
}

#[test]
#[should_panic]
fn test_parse_full_invalid_scheme() {
    let input = Bytes::from_static(b"htt://example.com");
    let _ = parse_full(input).unwrap();
}

#[test]
#[should_panic]
fn test_parse_full_invalid_authority() {
    let input = Bytes::from_static(b"http://");
    let _ = parse_full(input).unwrap();
}

#[test]
fn test_parse_full_empty_uri() {
    let input = Bytes::from_static(b"");
    let result = parse_full(input);
    assert!(result.is_err());
}

