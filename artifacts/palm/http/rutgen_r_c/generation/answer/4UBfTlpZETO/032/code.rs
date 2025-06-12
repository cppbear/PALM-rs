// Answer 0

#[test]
fn test_parse_with_invalid_uri_character() {
    let input: &[u8] = b"example:host@value%20"; // '%20' is an invalid character after userinfo
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_with_unmatched_brackets() {
    let input: &[u8] = b"[example:host"; // unmatched '[' bracket
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_with_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030"; // Too many colons
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_with_empty_userinfo() {
    let input: &[u8] = b"example@"; // '@' with nothing after
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_with_percent_character_not_in_userinfo() {
    let input: &[u8] = b"example:host@value%"; // invalid percent outside of userinfo
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

