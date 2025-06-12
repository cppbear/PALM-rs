// Answer 0

fn test_parse_invalid_authority_too_many_colons() {
    let input = b"localhost:8080:3030";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

fn test_parse_invalid_authority_brackets_mismatch() {
    let input = b"[::1:8080";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

fn test_parse_invalid_authority_empty_at_sign() {
    let input = b"localhost:@8080";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

fn test_parse_invalid_authority_percent_after_userinfo() {
    let input = b"user:pass@localhost%:8080";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

fn test_parse_invalid_authority_invalid_character() {
    let input = b"localhost:80#extra";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

fn test_parse_invalid_authority_mixed_brackets() {
    let input = b"[localhost:8080]";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

fn test_parse_invalid_authority_too_long() {
    let input = b"localhost:80:80:80:80:80:80:80:80";
    let result = Authority::parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

