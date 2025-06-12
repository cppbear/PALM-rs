// Answer 0

#[test]
fn test_parse_invalid_utf8_character() {
    let input = vec![255u8]; // Invalid UTF-8 byte
    let result = parse(&input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_uri_character() {
    let input = vec![0u8]; // Null byte, which is invalid in a URI
    let result = parse(&input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_percent_character() {
    let input = b"abc%xyz".to_vec(); // An invalid percent position since 'x' is invalid
    let result = parse(&input);
    assert!(result.is_err());
}

#[test]
fn test_parse_extra_at_sign() {
    let input = b"user@info@host".to_vec(); // Two '@' characters
    let result = parse(&input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_ipv6_brackets() {
    let input = b"::1:80".to_vec(); // Starting with double colons and should be invalid
    let result = parse(&input);
    assert!(result.is_err());
}

