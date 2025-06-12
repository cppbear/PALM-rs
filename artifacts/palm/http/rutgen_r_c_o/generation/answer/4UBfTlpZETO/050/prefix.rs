// Answer 0

#[test]
fn test_parse_with_multiple_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_with_brackets_and_invalid_end_bracket() {
    let input: &[u8] = b"[::1]:80@";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_with_valid_chars_exceeding_colons() {
    let input: &[u8] = b"example:localhost:8080:3030";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_with_percent_sign_after_userinfo() {
    let input: &[u8] = b"user:pass@localhost%:80";
    let authority = Authority::empty();
    authority.parse(input);
}

