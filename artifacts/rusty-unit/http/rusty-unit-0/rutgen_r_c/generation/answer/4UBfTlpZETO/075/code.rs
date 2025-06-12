// Answer 0

#[test]
fn test_parse_invalid_authority_invalid_uri_char() {
    let input: &[u8] = b"\xFF"; // Invalid character
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidUriChar.into()));
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input: &[u8] = b"[localhost"; // Unmatched starting bracket
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_colon_count_exceeded() {
    let input: &[u8] = b"localhost:8080:3030"; // More than 1 colon
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_empty_after_at_sign() {
    let input: &[u8] = b"username@"; // Empty after '@'
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_found() {
    let input: &[u8] = b"localhost%20"; // Percent following userinfo
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_valid_but_unresolvable() {
    let input: &[u8] = b"::1"; // IPv6 format but unresolvable
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

