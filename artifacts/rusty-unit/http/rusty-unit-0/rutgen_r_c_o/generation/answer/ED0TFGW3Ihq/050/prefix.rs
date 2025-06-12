// Answer 0

#[test]
fn test_parse_exact_invalid_scheme() {
    let input = b"invalid_scheme"; // length is less than 64 and does not match http or https
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_scheme_with_special_char() {
    let input = b"invalid:scheme"; // contains ':' which maps to 0 in SCHEME_CHARS
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_too_long_invalid_scheme() {
    let input: &[u8] = b"invalid_scheme_invalid_scheme_invalid_scheme_invalid_scheme"; // 64 bytes long
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_contains_invalid_char() {
    let input: &[u8] = b"inv@lid_scheme_with_invalid_character"; // contains '@' which maps to 0 in SCHEME_CHARS
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_with_multiple_invalid_chars() {
    let input: &[u8] = b"fail@123:data"; // contains '@' and ':' which map to 0 in SCHEME_CHARS
    let _ = Scheme2::parse_exact(input);
}

