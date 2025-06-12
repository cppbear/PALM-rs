// Answer 0

#[test]
fn test_parse_valid_uri_with_single_colon() {
    let input: &[u8] = b"example.com:80/";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_uri_with_single_colon_and_question_mark() {
    let input: &[u8] = b"example.com:80?query=1";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_uri_with_single_colon_and_hash() {
    let input: &[u8] = b"example.com:80#fragment";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_uri_with_ipv6_format() {
    let input: &[u8] = b"[2001:db8::1]:8080/";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_uri_with_userinfo_and_single_colon() {
    let input: &[u8] = b"user:pass@example.com:80/";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_valid_uri_max_length() {
    let input: &[u8] = b"http://example.com:80/path?query#fragment";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_uri_with_no_colon() {
    let input: &[u8] = b"example.com/";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_uri_with_single_colon_and_no_path() {
    let input: &[u8] = b"example.com:80";
    let authority = Authority::empty();
    authority.parse(input);
}

