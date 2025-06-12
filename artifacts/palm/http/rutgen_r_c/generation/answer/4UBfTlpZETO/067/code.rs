// Answer 0

#[test]
fn test_parse_valid_authority() {
    let valid_authority = b"username:password@host.com:8080/path";
    let result = Authority::parse(valid_authority);
    assert_eq!(result, Ok(26));
}

#[test]
fn test_parse_valid_ip_v6_authority() {
    let valid_ip_v6 = b"[2001:db8::1]:80";
    let result = Authority::parse(valid_ip_v6);
    assert_eq!(result, Ok(13));
}

#[test]
fn test_parse_valid_authority_without_port() {
    let valid_authority_no_port = b"host.com/path";
    let result = Authority::parse(valid_authority_no_port);
    assert_eq!(result, Ok(12));
}

#[test]
fn test_parse_invalid_authority_extra_colon() {
    let invalid_authority = b"host.com:8080:3030/path";
    let result = Authority::parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_missing_end_bracket() {
    let invalid_authority = b"[2001:db8::1:80";
    let result = Authority::parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_at_sign_without_username() {
    let invalid_authority = b"@host.com:8080/path";
    let result = Authority::parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_percent() {
    let invalid_authority = b"username:pass%word@host.com:8080/path";
    let result = Authority::parse(invalid_authority);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_invalid_char() {
    let invalid_authority = b"username:pass#word@host.com:8080/path";
    let result = Authority::parse(invalid_authority);
    assert!(result.is_err());
}

