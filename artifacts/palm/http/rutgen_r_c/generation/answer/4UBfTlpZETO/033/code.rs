// Answer 0

#[test]
fn test_parse_invalid_authority_missing_end_bracket() {
    let input: &[u8] = b"example.com[";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_multiple_colons() {
    let input: &[u8] = b"example.com:80:90";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_at_sign_without_host() {
    let input: &[u8] = b"@:80";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_in_hostname() {
    let input: &[u8] = b"example.com%20";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_unexpected_percent() {
    let input: &[u8] = b"example.com:user%20info";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

