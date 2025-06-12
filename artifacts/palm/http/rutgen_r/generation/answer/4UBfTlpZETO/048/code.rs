// Answer 0

#[test]
fn test_parse_valid_no_colons() {
    let input: &[u8] = b"localhost";
    let result = http::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_valid_single_colon() {
    let input: &[u8] = b"localhost:8080";
    let result = http::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_valid_with_ipv6() {
    let input: &[u8] = b"[2001:db8::1]:80";
    let result = http::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_valid_with_userinfo() {
    let input: &[u8] = b"user:pass@localhost";
    let result = http::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_valid_multiple_segments_before_separator() {
    let input: &[u8] = b"example.com:80";
    let result = http::parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_invalid_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = http::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_unbalanced_brackets() {
    let input: &[u8] = b"[localhost";
    let result = http::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_unbalanced_brackets_end() {
    let input: &[u8] = b"localhost]";
    let result = http::parse(input);
    assert!(result.is_err());
}

