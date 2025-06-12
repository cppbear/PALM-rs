// Answer 0

#[test]
fn test_parse_non_empty_valid_uri() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"example.com:80");
    assert!(result.is_ok());
}

#[test]
fn test_parse_non_empty_valid_uri_with_brackets() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"[2001:db8::1]:8080");
    assert!(result.is_ok());
}

#[test]
fn test_parse_non_empty_invalid_uri_empty_string() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"");
    assert_eq!(result, Err(InvalidUri(ErrorKind::Empty)));
}

#[test]
fn test_parse_non_empty_invalid_uri_too_many_colons() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"example:com:80:90");
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidAuthority)));
}

#[test]
fn test_parse_non_empty_invalid_uri_unmatched_brackets() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"[2001:db8::1:8080");
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidAuthority)));
}

#[test]
fn test_parse_non_empty_invalid_uri_at_sign_after_end() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"example.com@:80");
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidAuthority)));
}

#[test]
fn test_parse_non_empty_invalid_uri_percentage_character() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(b"example.com%80");
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidAuthority)));
}

