// Answer 0

#[test]
fn test_parse_valid_authority_with_single_colon() {
    let input: &[u8] = b"example.com:8080";
    let result = parse(input);
    assert_eq!(result, Ok(14));
}

#[test]
fn test_parse_empty_string() {
    let input: &[u8] = b"";
    let result = parse(input);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_parse_authority_with_ipv6() {
    let input: &[u8] = b"[2001:db8::1]:80";
    let result = parse(input);
    assert_eq!(result, Ok(16));
}

#[test]
fn test_parse_authority_with_userinfo_and_single_colon() {
    let input: &[u8] = b"user:pass@example.com:8080";
    let result = parse(input);
    assert_eq!(result, Ok(22));
}

#[test]
#[should_panic]
fn test_parse_authority_with_multiple_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = parse(input);
    // Expect panic due to too many colons
    assert_eq!(result.is_ok(), false);
}

#[test]
#[should_panic]
fn test_parse_authority_with_mismatched_brackets() {
    let input: &[u8] = b"[2001:db8::1:80";
    let result = parse(input);
    // Expect panic due to mismatched brackets
    assert_eq!(result.is_ok(), false);
}

#[test]
fn test_parse_authority_with_at_sign() {
    let input: &[u8] = b"user@example.com:8080";
    let result = parse(input);
    assert_eq!(result, Ok(20));
}

