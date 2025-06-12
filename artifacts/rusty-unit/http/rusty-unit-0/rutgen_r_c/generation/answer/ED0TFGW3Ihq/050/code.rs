// Answer 0

#[test]
fn test_parse_exact_invalid_scheme_too_long() {
    let input: &[u8] = b"invalid_uri_scheme_that_is_way_too_long_to_be_valid";
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::SchemeTooLong)));
}

#[test]
fn test_parse_exact_invalid_scheme_with_invalid_characters() {
    let input: &[u8] = b"invalid:scheme"; // contains a ':'
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

#[test]
fn test_parse_exact_invalid_scheme_with_invalid_characters_2() {
    let input: &[u8] = b"invalid/scheme"; // contains a '/'
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

#[test]
fn test_parse_exact_invalid_scheme_with_invalid_character_3() {
    let input: &[u8] = b"invalid#scheme"; // contains a '#'
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

#[test]
fn test_parse_exact_invalid_scheme_character_beyond_limit() {
    let input: &[u8] = &[0xFF]; // invalid byte outside of valid scheme characters
    let result = Scheme2::<()>::parse_exact(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::InvalidScheme)));
}

