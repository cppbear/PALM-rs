// Answer 0

#[test]
fn test_parse_valid_authority() {
    let input: &[u8] = b"example.com:80"; // valid authority with one colon
    assert_eq!(parse(input), Ok(16)); // the length of the input
}

#[test]
fn test_parse_authority_with_userinfo() {
    let input: &[u8] = b"user:pass@example.com:80"; // valid authority with userinfo
    assert_eq!(parse(input), Ok(23)); // the length of the input
}

#[test]
fn test_parse_ipv4_address() {
    let input: &[u8] = b"192.168.1.1:8080"; // valid IPv4 address with port
    assert_eq!(parse(input), Ok(15)); // the length of the input
}

#[test]
fn test_parse_ipv6_address() {
    let input: &[u8] = b"[2001:db8::1]:8080"; // valid IPv6 address with port
    assert_eq!(parse(input), Ok(18)); // the length of the input
}

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030"; // more than one port colon
    assert!(parse(input).is_err());
}

#[test]
fn test_parse_invalid_authority_empty_post_at_sign() {
    let input: &[u8] = b"user:pass@example.com@"; // nothing after '@'
    assert!(parse(input).is_err());
}

#[test]
fn test_parse_invalid_authority_percent_without_userinfo() {
    let input: &[u8] = b"user:pass%#@example.com"; // percent after userinfo
    assert!(parse(input).is_err());
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input: &[u8] = b"[2001:db8::1:8080"; // unmatched opening bracket
    assert!(parse(input).is_err());
}

