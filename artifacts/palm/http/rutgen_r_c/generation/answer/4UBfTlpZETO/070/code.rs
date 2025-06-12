// Answer 0

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input = b"localhost:8080:3030"; // More than one colon
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_missing_end_bracket() {
    let input = b"[::1"; // Missing a closing bracket
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_missing_start_bracket() {
    let input = b"::1]"; // Missing an opening bracket
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_at_sign_without_following() {
    let input = b"username:password@"; // At sign with nothing after
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_sign() {
    let input = b"localhost%"; // Percent sign found outside userinfo
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

