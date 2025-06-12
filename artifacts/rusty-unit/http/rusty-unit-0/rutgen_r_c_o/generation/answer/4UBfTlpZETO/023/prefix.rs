// Answer 0

#[test]
fn test_parse_invalid_authority_with_start_bracket_and_multiple_colons() {
    let input: &[u8] = b"example[:80:somewhere]";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_invalid_authority_with_end_bracket_and_invalid_colons() {
    let input: &[u8] = b"example[invalid]:[something]:80";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_invalid_authority_with_percent_symbol() {
    let input: &[u8] = b"example[%]:80";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_invalid_authority_with_brackets_and_colons() {
    let input: &[u8] = b"example[valid]:[invalid]:8080";
    let authority = Authority::empty();
    authority.parse(input);
}

#[test]
fn test_parse_invalid_authority_no_matching_bracket() {
    let input: &[u8] = b"example[valid:80";
    let authority = Authority::empty();
    authority.parse(input);
}

