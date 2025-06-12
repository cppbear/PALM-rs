// Answer 0

#[test]
fn test_parse_valid_authority_with_one_colon() {
    let input: &[u8] = b"example.com:80";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_authority_with_ipv6() {
    let input: &[u8] = b"[abcd:ef01:2345:6789:abcd:ef01:2345:6789]:80";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_authority_with_userinfo() {
    let input: &[u8] = b"user:pass@example.com:80";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_authority_with_path() {
    let input: &[u8] = b"example.com:80/path";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_authority_with_empty_host() {
    let input: &[u8] = b":80";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_authority_with_ipv6_and_path() {
    let input: &[u8] = b"[abcd:ef01:2345:6789:abcd:ef01:2345:6789]/path";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_authority_with_special_chars() {
    let input: &[u8] = b"user+name:pass@host:8080/path";
    let authority = Authority::empty();
    authority.parse(input);
}

