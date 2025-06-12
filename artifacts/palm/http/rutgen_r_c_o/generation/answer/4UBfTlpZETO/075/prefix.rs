// Answer 0

#[test]
#[should_panic]
fn test_parse_invalid_authority_with_only_percent() {
    let input: &[u8] = b"%";
    Authority::parse(input);
}

#[test]
#[should_panic]
fn test_parse_invalid_authority_with_percent_at_end() {
    let input: &[u8] = b"example.com%";
    Authority::parse(input);
}

#[test]
#[should_panic]
fn test_parse_invalid_authority_with_open_bracket_and_percent() {
    let input: &[u8] = b"[example%";
    Authority::parse(input);
}

#[test]
#[should_panic]
fn test_parse_invalid_authority_with_multiple_colons() {
    let input: &[u8] = b"example:80:90";
    Authority::parse(input);
}

#[test]
#[should_panic]
fn test_parse_invalid_authority_with_at_sign_followed_by_nothing() {
    let input: &[u8] = b"example.com@";
    Authority::parse(input);
}

