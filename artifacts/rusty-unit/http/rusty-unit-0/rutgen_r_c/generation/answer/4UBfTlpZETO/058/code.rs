// Answer 0

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030"; // This should trigger the colon count condition.
    assert_eq!(Authority::parse(input).unwrap_err().0, ErrorKind::InvalidAuthority);
}

#[test]
fn test_parse_invalid_authority_missing_brackets() {
    let input: &[u8] = b"[localhost"; // This should trigger the start and end bracket condition.
    assert_eq!(Authority::parse(input).unwrap_err().0, ErrorKind::InvalidAuthority);
}

#[test]
fn test_parse_invalid_authority_empty_at_sign() {
    let input: &[u8] = b"username:@localhost"; // The @ is at the end without any following authority.
    assert_eq!(Authority::parse(input).unwrap_err().0, ErrorKind::InvalidAuthority);
}

#[test]
fn test_parse_invalid_authority_with_percent_sign() {
    let input: &[u8] = b"username%40localhost"; // A percent sign present should trigger an error.
    assert_eq!(Authority::parse(input).unwrap_err().0, ErrorKind::InvalidAuthority);
}

#[test]
fn test_parse_invalid_uri_character() {
    let input: &[u8] = b"localhost\x7f"; // Non-valid URI character (DEL character)
    assert_eq!(Authority::parse(input).unwrap_err().0, ErrorKind::InvalidUriChar);
}

#[test]
fn test_parse_valid_authority_with_brackets() {
    let input: &[u8] = b"[::1]:80"; // This is a valid IPv6 and should return the length without an error.
    assert!(Authority::parse(input).is_ok());
}

