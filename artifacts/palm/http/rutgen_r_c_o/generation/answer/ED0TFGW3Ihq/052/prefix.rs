// Answer 0

#[test]
fn test_parse_exact_valid_scheme() {
    let input = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklm";
    let result = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_multiple_chars() {
    let input = b"validschemecharacters";
    let result = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_boundary_length() {
    let input = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklm"; // length == MAX_SCHEME_LEN
    let result = Scheme2::<()>::parse_exact(input);
}

