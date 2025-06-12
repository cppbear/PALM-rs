// Answer 0

#[test]
fn test_parse_invalid_authority_start_bracket_without_end_bracket() {
    let input: &[u8] = b"[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210"; // Missing closing bracket
    let result = parse(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorKind::InvalidAuthority.into());
}

#[test]
fn test_parse_invalid_authority_excess_colons() {
    let input: &[u8] = b"localhost:8080:3030"; // More than one colon
    let result = parse(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorKind::InvalidAuthority.into());
}

#[test]
fn test_parse_invalid_authority_at_sign_with_trailing() {
    let input: &[u8] = b"username@"; // At sign at the end with nothing after
    let result = parse(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorKind::InvalidAuthority.into());
}

#[test]
fn test_parse_invalid_authority_percent_char() {
    let input: &[u8] = b"localhost%20"; // Contains percent-encoding without valid context
    let result = parse(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorKind::InvalidAuthority.into());
}

