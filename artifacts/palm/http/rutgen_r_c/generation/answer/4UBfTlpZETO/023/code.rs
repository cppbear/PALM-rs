// Answer 0

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input = b"localhost:8080:3030";
    let authority = Authority::empty();
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_at_sign_without_userinfo() {
    let input = b"test@";
    let authority = Authority::empty();
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input = b"[::1";
    let authority = Authority::empty();
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_invalid_percentage() {
    let input = b"test%user@localhost";
    let authority = Authority::empty();
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_invalid_character() {
    let input = b"test&localhost";
    let authority = Authority::empty();
    let result = authority.parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidUriChar.into()));
}

