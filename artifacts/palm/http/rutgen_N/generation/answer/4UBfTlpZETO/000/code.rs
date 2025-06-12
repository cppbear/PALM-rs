// Answer 0

#[test]
fn test_parse_valid_uri() {
    let input = b"example.com:80";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), input.len());
}

#[test]
fn test_parse_valid_ipv6_uri() {
    let input = b"[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:8080";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), input.len());
}

#[test]
fn test_parse_too_many_colons() {
    let input = b"localhost:8080:9090";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_character() {
    let input = b"invalid_uri@";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_ipv6_brackets() {
    let input = b"[2605:dead:babe::1:80";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_at_sign_no_following() {
    let input = b"@";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_percent_not_after_userinfo() {
    let input = b"user:pass@domain.com%20";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_input() {
    let input: &[u8] = b"";
    let result = parse(input);
    assert!(result.is_err());
}

