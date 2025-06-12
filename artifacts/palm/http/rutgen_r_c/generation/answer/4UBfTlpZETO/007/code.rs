// Answer 0

#[test]
fn test_parse_valid_authority() {
    let input: &[u8] = b"example.com"; // No special characters, should be valid
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_with_port() {
    let input: &[u8] = b"example.com:80"; // Valid with port
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_ipv6() {
    let input: &[u8] = b"[2001:db8::1]:8080"; // Valid IPv6 with port
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_with_userinfo() {
    let input: &[u8] = b"user:pass@example.com"; // Valid with userinfo
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_with_query() {
    let input: &[u8] = b"example.com?query=param"; // Valid with query
    let result = Authority::parse(input);
    assert_eq!(result, Ok(11)); // up to '?' character
}

#[test]
fn test_parse_with_hash() {
    let input: &[u8] = b"example.com#section"; // Valid with fragment
    let result = Authority::parse(input);
    assert_eq!(result, Ok(11)); // up to '#' character
}

#[test]
fn test_parse_invalid_character() {
    let input: &[u8] = b"example.com%x"; // Invalid because of '%'
    let result = Authority::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_multiple_colons() {
    let input: &[u8] = b"example.com:80:90"; // Invalid, more than one colon
    let result = Authority::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_unmatched_brackets() {
    let input: &[u8] = b"[example.com"; // Invalid, unmatched '['
    let result = Authority::parse(input);
    assert!(result.is_err());

    let input: &[u8] = b"example.com]"; // Invalid, unmatched ']'
    let result = Authority::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_string() {
    let input: &[u8] = b""; // Edge case, should return 0
    let result = Authority::parse(input);
    assert_eq!(result, Ok(0));
}

