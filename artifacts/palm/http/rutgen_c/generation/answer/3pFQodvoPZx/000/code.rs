// Answer 0

#[test]
fn test_parse_non_empty_valid() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"example.com");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 11);
}

#[test]
#[should_panic(expected = "InvalidUri(ErrorKind::Empty)")]
fn test_parse_non_empty_empty() {
    let authority = Authority::empty();
    authority.parse_non_empty(b"").unwrap();
}

#[test]
fn test_parse_non_empty_invalid_uri_char() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"example?.com");
    assert!(result.is_err());
}

#[test]
fn test_parse_non_empty_invalid_authority() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"invalid_authority@host:123");
    assert!(result.is_err());
}

#[test]
fn test_parse_non_empty_single_colon() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"host:80");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 8);
}

#[test]
fn test_parse_non_empty_too_many_colons() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"host:80:90");
    assert!(result.is_err());
}

