// Answer 0

#[test]
fn test_parse_invalid_authority_unclosed_bracket() {
    let input: &[u8] = b"localhost:[80";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_invalid_character() {
    let input: &[u8] = b"localhost:[80%";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_no_closing_bracket() {
    let input: &[u8] = b"example.com:[port";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_multiple_colons() {
    let input: &[u8] = b"localhost:[::1:80:90";
    let result = Authority::parse(input);
}

#[test]
fn test_parse_invalid_authority_invalid_percent() {
    let input: &[u8] = b"host:[::1]:90%stuff";
    let result = Authority::parse(input);
}

