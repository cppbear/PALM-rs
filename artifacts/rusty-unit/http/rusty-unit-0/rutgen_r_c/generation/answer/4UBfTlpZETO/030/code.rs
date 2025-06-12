// Answer 0

#[test]
fn test_parse_valid_authority() {
    let input: &[u8] = b"example.com:8080";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_ipv6_without_port() {
    let input: &[u8] = b"[2001:db8::1]";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_ipv6_with_port() {
    let input: &[u8] = b"[2001:db8::1]:8080";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_with_userinfo_and_host() {
    let input: &[u8] = b"user:pass@example.com";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = Authority::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_multiple_at_signs() {
    let input: &[u8] = b"user:pass@user2:pass@example.com";
    let result = Authority::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input: &[u8] = b"[example.com";
    let result = Authority::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_empty() {
    let input: &[u8] = b"";
    let result = Authority::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_percent_in_host() {
    let input: &[u8] = b"example%.com";
    let result = Authority::parse(input);
    assert!(result.is_err());
}

