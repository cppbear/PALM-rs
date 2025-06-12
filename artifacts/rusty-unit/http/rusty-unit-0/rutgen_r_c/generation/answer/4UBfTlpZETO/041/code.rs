// Answer 0

#[test]
fn test_parse_exceeding_max_colons() {
    let input = b"localhost:8080:3030"; // This string should trigger the error due to multiple colons.
    let result = Authority::parse(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidAuthority)));
}

#[test]
fn test_parse_single_colon() {
    let input = b"localhost:8080"; // This string has a valid single colon.
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len())); // Should return the length of the input since it is valid.
}

#[test]
fn test_parse_multiple_colons() {
    let input = b"my:host:with:multiple:colons"; // This string has multiple colons.
    let result = Authority::parse(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidAuthority))); // Should trigger error due to multiple colons.
}

#[test]
fn test_parse_additional_invalid_characters() {
    let input = b"my@host:80"; // This string has an '@' character, which should not be at the end.
    let result = Authority::parse(input);
    assert_eq!(result, Ok(input.len())); // Should return the length of the input since it is valid as there is something after '@'.
}

#[test]
fn test_parse_bracket_mismatch() {
    let input = b"[ipv6::1:2:3:4:5:6:7:8"; // This string has a start bracket but no corresponding end bracket.
    let result = Authority::parse(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidAuthority))); // Should trigger error due to mismatched brackets.
}

