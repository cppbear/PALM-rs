// Answer 0

#[test]
fn test_parse_valid_authority() {
    let input: &[u8] = b"example.com:80";
    let result = parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_valid_ipv6_authority() {
    let input: &[u8] = b"[2001:db8::1]:80";
    let result = parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_valid_userinfo_authority() {
    let input: &[u8] = b"user:pass@example.com:80";
    let result = parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_invalid_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030"; // Invalid: too many colons
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_unmatched_brackets() {
    let input: &[u8] = b"[example.com"; // Invalid: unmatched '['
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_end_bracket_without_start() {
    let input: &[u8] = b"example.com]:80"; // Invalid: unmatched ']'
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_at_sign_with_no_following() {
    let input: &[u8] = b"example.com:@80"; // Invalid: '@' must have something after
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_percent_sign() {
    let input: &[u8] = b"example.com%80"; // Invalid: '%' in a bad context
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_empty_authority() {
    let input: &[u8] = b""; // Invalid: empty input
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_valid_with_ipv6_zone_id() {
    let input: &[u8] = b"[2001:db8::1%zone]:80"; // Valid: contains zone ID
    let result = parse(input);
    assert_eq!(result, Ok(input.len()));
}

