// Answer 0

#[test]
fn test_parse_non_empty_valid_uri() {
    let input = b"example.com";
    let _ = Authority::parse_non_empty(input);
}

#[test]
fn test_parse_non_empty_valid_uri_with_port() {
    let input = b"example.com:8080";
    let _ = Authority::parse_non_empty(input);
}

#[test]
fn test_parse_non_empty_valid_uri_with_userinfo() {
    let input = b"user:pass@example.com";
    let _ = Authority::parse_non_empty(input);
}

#[test]
fn test_parse_non_empty_valid_uri_with_ipv6() {
    let input = b"[2001:db8::1]:80";
    let _ = Authority::parse_non_empty(input);
}

#[test]
fn test_parse_non_empty_valid_uri_with_special_chars() {
    let input = b"example.com/path?query#fragment";
    let _ = Authority::parse_non_empty(input);
}

#[test]
fn test_parse_non_empty_valid_uri_with_empty_path() {
    let input = b"example.com/";
    let _ = Authority::parse_non_empty(input);
}

#[test]
#[should_panic]
fn test_parse_non_empty_empty_input() {
    let input: &[u8] = &[];
    let _ = Authority::parse_non_empty(input);
}

