// Answer 0

#[test]
fn test_parse_empty() {
    let input: &[u8] = b"";
    Authority::parse(input);
}

#[test]
fn test_parse_single_colon() {
    let input: &[u8] = b"localhost:80";
    Authority::parse(input);
}

#[test]
fn test_parse_only_colon() {
    let input: &[u8] = b":";
    Authority::parse(input);
}

#[test]
fn test_parse_valid_char() {
    let input: &[u8] = b"a";
    Authority::parse(input);
}

#[test]
fn test_parse_invalid_char() {
    let input: &[u8] = b"%";
    Authority::parse(input);
}

