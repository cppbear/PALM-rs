// Answer 0

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input: &[u8] = b"example.com[";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_unmatched_end_bracket() {
    let input: &[u8] = b"example.com]:80";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_multiple_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_at_sign_without_follows() {
    let input: &[u8] = b"example.com@";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_in_hostname() {
    let input: &[u8] = b"example%.com";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_single_bracket() {
    let input: &[u8] = b"example.com[foo";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_bracket_without_colon() {
    let input: &[u8] = b"example.com[:80";
    let result = parse(input);
    assert_eq!(result, Err(ErrorKind::InvalidAuthority.into()));
}

