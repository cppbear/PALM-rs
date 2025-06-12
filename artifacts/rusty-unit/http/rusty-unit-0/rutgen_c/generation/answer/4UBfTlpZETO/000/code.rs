// Answer 0

#[test]
fn test_parse_valid_uri() {
    let authority = Authority::empty();
    let result = authority.parse(b"localhost:8080");
    assert_eq!(result.unwrap(), 16);
}

#[test]
fn test_parse_invalid_uri_with_multiple_colons() {
    let authority = Authority::empty();
    let result = authority.parse(b"localhost:8080:3030");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_with_unmatched_brackets() {
    let authority = Authority::empty();
    let result = authority.parse(b"[::1:80");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_with_invalid_character() {
    let authority = Authority::empty();
    let result = authority.parse(b"localhost:80\xFF");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_with_at_sign_at_end() {
    let authority = Authority::empty();
    let result = authority.parse(b"localhost:80@");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_with_percent_sign_after_userinfo() {
    let authority = Authority::empty();
    let result = authority.parse(b"user:pass@localhost:80%");
    assert!(result.is_err());
}

