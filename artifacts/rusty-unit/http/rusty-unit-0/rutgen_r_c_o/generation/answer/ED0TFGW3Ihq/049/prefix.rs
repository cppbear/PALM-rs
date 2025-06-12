// Answer 0

#[test]
fn test_parse_exact_valid_other() {
    let input: &[u8] = b"validscheme1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; // Length = 64
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_char_colon() {
    let input: &[u8] = b"invalid:scheme"; // Contains ':', should trigger InvalidScheme
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_char_zero() {
    let input: &[u8] = b"invalid\x00scheme"; // Contains null byte, should trigger InvalidScheme
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_scheme_too_long() {
    let input: &[u8] = b"thisschemeiswaytoolongthisschemeiswaytoolong"; // Length = 65
    let _ = Scheme2::parse_exact(input);
}

#[test]
fn test_parse_exact_invalid_scheme_not_standard() {
    let input: &[u8] = b"1234567890!@#$%^&*()-_=+[]{}|;:'\",<>" ; // No valid scheme characters
    let _ = Scheme2::parse_exact(input);
}

