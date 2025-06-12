// Answer 0

#[test]
fn test_parse_valid_uri_with_one_colon() {
    let input: &[u8] = b"example.com:80";
    let result = parse(input);
    assert_eq!(result, Ok(15));
}

#[test]
fn test_parse_valid_uri_ipv6_format() {
    let input: &[u8] = b"[2001:db8::1]:80";
    let result = parse(input);
    assert_eq!(result, Ok(15));
}

#[test]
fn test_parse_invalid_uri_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_unbalanced_brackets() {
    let input: &[u8] = b"[example.com:80";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_with_percent() {
    let input: &[u8] = b"example.com%20:80";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_empty_input() {
    let input: &[u8] = b"";
    let result = parse(input);
    assert!(result.is_err());
}

