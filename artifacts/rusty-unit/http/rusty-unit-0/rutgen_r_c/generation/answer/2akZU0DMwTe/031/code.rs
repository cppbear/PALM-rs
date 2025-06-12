// Answer 0

#[test]
fn test_parse_scheme2_none() {
    let input: &[u8] = b"abcd:efgh"; // Length is 8, character at index 4 is ':', but s[5..7] != "//".
    let result = Scheme2::<usize>::parse(input);
    assert_eq!(result, Ok(Scheme2::None));
}

#[test]
fn test_parse_scheme2_other() {
    let input: &[u8] = b"myScheme://example.com"; // Length is 15, valid scheme character.
    let result = Scheme2::<usize>::parse(input);
    assert!(matches!(result, Ok(Scheme2::Other(_))));
}

#[test]
fn test_parse_scheme2_invalid_scheme() {
    let input: &[u8] = b"invalidScheme://"; // Length is 15, valid scheme character.
    let result = Scheme2::<usize>::parse(input);
    assert!(matches!(result, Ok(Scheme2::Other(_))));
}

#[test]
fn test_parse_scheme2_too_long() {
    let long_scheme = b"verylongschemelimitedto64charactersandmorethanfiftysixcharacters:";
    let result = Scheme2::<usize>::parse(long_scheme);
    assert!(result.is_err());
}

