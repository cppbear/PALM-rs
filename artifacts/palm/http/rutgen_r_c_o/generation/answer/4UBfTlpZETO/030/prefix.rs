// Answer 0

#[test]
fn test_parse_valid_ipv6_authority() {
    let input: &[u8] = b"[::1:8080]";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_valid_ipv6_authority_with_localhost() {
    let input: &[u8] = b"[localhost:8080]";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_valid_authority_with_no_brackets() {
    let input: &[u8] = b"localhost:8080";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_valid_authority_with_single_colon() {
    let input: &[u8] = b"[::1:80]";
    let _ = Authority::parse(input);
}

#[test]
fn test_parse_valid_authority_with_only_ipv6() {
    let input: &[u8] = b"[2001:db8::1]:80";
    let _ = Authority::parse(input);
}

