// Answer 0

#[test]
fn test_parse_invalid_authority_due_to_multiple_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_due_to_unmatched_brackets() {
    let input: &[u8] = b"[::1:80";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_due_to_at_sign_position() {
    let input: &[u8] = b"username:password@";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_due_to_percent_after_userinfo() {
    let input: &[u8] = b"username@hostname%20";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

