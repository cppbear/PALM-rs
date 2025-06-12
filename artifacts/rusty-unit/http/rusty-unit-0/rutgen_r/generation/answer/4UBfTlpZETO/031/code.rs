// Answer 0

#[test]
fn test_parse_invalid_authority_with_unmatched_bracket() {
    let input: &[u8] = b"[example.com"; // Starts bracket but does not have an end bracket
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_double_colons() {
    let input: &[u8] = b"localhost::8080"; // Contains multiple colons
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_percent_after_userinfo() {
    let input: &[u8] = b"user@host%20"; // Percent character follows the userinfo
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_empty_userinfo() {
    let input: &[u8] = b"@host"; // Empty userinfo before the host
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_invalid_character() {
    let input: &[u8] = b"host#query"; // Contains '#' which should terminate the authority
    let result = parse(input);
    assert!(result.is_err());
}

