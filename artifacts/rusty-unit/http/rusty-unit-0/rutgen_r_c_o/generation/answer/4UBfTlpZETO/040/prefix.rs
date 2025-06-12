// Answer 0

#[test]
fn test_parse_valid_authority_with_one_colon() {
    let input: &[u8] = b"localhost:8080";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_valid_authority_without_colons() {
    let input: &[u8] = b"example.com";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_valid_authority_with_balanced_brackets() {
    let input: &[u8] = b"[2001:db8::1]:80";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_valid_authority_with_percent_encoding_in_userinfo() {
    let input: &[u8] = b"user%20name@host.com";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_valid_ipv6_authority_with_one_colon() {
    let input: &[u8] = b"[::1]:3000";
    let result = Authority::parse(input);
}

