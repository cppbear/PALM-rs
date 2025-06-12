// Answer 0

#[test]
fn test_parse_valid_authority_with_single_colon() {
    let input: &[u8] = b"localhost:80";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(12));
}

#[test]
fn test_parse_valid_ipv6_without_brackets() {
    let input: &[u8] = b"::1:80";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_parse_valid_authority_with_colon_and_no_brackets() {
    let input: &[u8] = b"example.com:8080";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(16));
}

#[test]
fn test_parse_valid_authority_with_no_userinfo() {
    let input: &[u8] = b"user:pass@example.com:80";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(31));
}

#[test]
fn test_parse_valid_authority_with_userinfo_and_ipv6() {
    let input: &[u8] = b"user:pass@[2001:db8::1]:80";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(28));
}

#[test]
fn test_parse_valid_authority_with_no_port() {
    let input: &[u8] = b"localhost";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(9));
}

#[test]
fn test_parse_valid_authority_with_empty_string() {
    let input: &[u8] = b"";
    let result = Authority::parse(input);
    assert_eq!(result, Ok(0));
}

