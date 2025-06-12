// Answer 0

#[test]
fn test_parse_exact_invalid_scheme_https() {
    let input = b"https://example.com";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_scheme_long() {
    let input = b"thisisaverylongschemethatexceedsthemaxlengthfortestcase";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_scheme_characters() {
    let input = b"invalid:scheme";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_scheme_space() {
    let input = b"invalid scheme";
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_scheme_empty() {
    let input = b"";
    let _ = Scheme2::parse_exact(input);
}

