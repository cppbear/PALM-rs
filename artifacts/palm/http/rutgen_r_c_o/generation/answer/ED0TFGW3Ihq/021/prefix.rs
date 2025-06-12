// Answer 0

#[test]
fn test_parse_exact_http() {
    let input: &[u8] = b"http";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_https() {
    let input: &[u8] = b"https";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_scheme() {
    let input: &[u8] = b"invalid_scheme";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_long_scheme() {
    let input: &[u8] = b"this_scheme_is_way_too_long_to_be_valid";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_contains_colon() {
    let input: &[u8] = b"invalid:scheme";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_empty() {
    let input: &[u8] = b"";
    let _ = Scheme2::parse_exact(input);
}

