// Answer 0

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let authority = Authority::empty();
    let input = b"localhost:8080:3030";
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_at_sign_end() {
    let authority = Authority::empty();
    let input = b"localhost:80@";
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_after_userinfo() {
    let authority = Authority::empty();
    let input = b"user:pass@localhost%80";
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let authority = Authority::empty();
    let input = b"[::1:80";
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_invalid_uri_char() {
    let authority = Authority::empty();
    let input = b"localhost:80\x00";
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidUriChar.into()));
}

