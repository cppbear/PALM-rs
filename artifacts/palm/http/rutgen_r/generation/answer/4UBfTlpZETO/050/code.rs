// Answer 0

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input: &[u8] = b"[::1:80";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_unmatched_start_bracket() {
    let input: &[u8] = b"::1]:80";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_at_sign_with_nothing_after() {
    let input: &[u8] = b"user@:80";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_after_userinfo() {
    let input: &[u8] = b"user:pass%info@localhost:80";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

