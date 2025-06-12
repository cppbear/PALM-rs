// Answer 0

#[test]
fn test_parse_valid_authority() {
    let authority = Authority::empty();
    let input: &[u8] = b"localhost:8080";
    let result = authority.parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_valid_ipv6_authority() {
    let authority = Authority::empty();
    let input: &[u8] = b"[2001:db8::1]:80";
    let result = authority.parse(input);
    assert_eq!(result, Ok(input.len()));
}

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let authority = Authority::empty();
    let input: &[u8] = b"localhost:8080:3030";
    let result = authority.parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let authority = Authority::empty();
    let input: &[u8] = b"[localhost:8080";
    let result = authority.parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_percent_in_host() {
    let authority = Authority::empty();
    let input: &[u8] = b"localhost%:8080";
    let result = authority.parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_empty_string() {
    let authority = Authority::empty();
    let input: &[u8] = b"";
    let result = authority.parse(input);
    assert!(result.is_err());
}

