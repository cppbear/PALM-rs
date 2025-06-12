// Answer 0

#[test]
fn test_parse_too_many_colons() {
    let input: &[u8] = b"test:user:info:with:too:many:colons:here:also:more";
    let result = http::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_single_colon() {
    let input: &[u8] = b"localhost:8080";
    let result = http::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_ipv6_with_max_colons() {
    let input: &[u8] = b"[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:80";
    let result = http::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_with_userinfo_and_colons() {
    let input: &[u8] = b"user:pass@localhost:8080";
    let result = http::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_invalid_authority_due_to_colons() {
    let input: &[u8] = b"test::too:many:colon:separators";
    let result = http::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_authority_with_exceeding_colons() {
    let input: &[u8] = b"authority:with:too:many:colons:here:even:more:colons";
    let result = http::parse(input);
    assert!(result.is_err());
}

