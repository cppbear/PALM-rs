// Answer 0

#[test]
fn test_parse_valid_authority() {
    let authority = Authority::empty();
    let input = b"localhost:8080";
    let result = authority.parse(input);
    assert_eq!(result, Ok(input.len() as usize));
}

#[test]
fn test_parse_valid_ipv6_authority() {
    let authority = Authority::empty();
    let input = b"[2001:db8::1]:80";
    let result = authority.parse(input);
    assert_eq!(result, Ok(input.len() as usize));
}

#[test]
fn test_parse_authority_with_no_colons() {
    let authority = Authority::empty();
    let input = b"example.com";
    let result = authority.parse(input);
    assert_eq!(result, Ok(input.len() as usize));
}

#[test]
fn test_parse_authority_with_single_colon() {
    let authority = Authority::empty();
    let input = b"example.com:8080";
    let result = authority.parse(input);
    assert_eq!(result, Ok(input.len() as usize));
}

#[test]
fn test_parse_authority_with_userinfo() {
    let authority = Authority::empty();
    let input = b"user:pass@example.com";
    let result = authority.parse(input);
    assert_eq!(result, Ok(input.len() as usize));
}

#[test]
fn test_parse_empty_string() {
    let authority = Authority::empty();
    let input = b"";
    let result = authority.parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_multiple_colons() {
    let authority = Authority::empty();
    let input = b"localhost:8080:3030";
    let result = authority.parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_percent() {
    let authority = Authority::empty();
    let input = b"user%:pass@example.com";
    let result = authority.parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_brackets() {
    let authority = Authority::empty();
    let input = b"[2001:db8::1:80";
    let result = authority.parse(input);
    assert!(result.is_err());
}

