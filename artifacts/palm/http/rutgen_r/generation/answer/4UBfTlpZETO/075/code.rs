// Answer 0

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_brackets_unmatched() {
    let input: &[u8] = b"[2001:0db8:85a3:0000:0000:8a2e:0370:7334@]";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_at_sign_with_no_following() {
    let input: &[u8] = b"localhost@";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_after_userinfo() {
    let input: &[u8] = b"username:pass@localhost%20";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_missing_brackets() {
    let input: &[u8] = b"[2001:0db8:85a3:0000:0000:8a2e:0370:7334";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

