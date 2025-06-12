// Answer 0

#[test]
fn test_parse_valid_case() {
    let input: &[u8] = b"localhost:8080";
    let result = parse(input);
    assert_eq!(result, Ok(15));
}

#[test]
fn test_parse_ipv6_case() {
    let input: &[u8] = b"[2001:db8::1]:80";
    let result = parse(input);
    assert_eq!(result, Ok(15));
}

#[test]
fn test_parse_with_userinfo() {
    let input: &[u8] = b"user:pass@localhost:8080";
    let result = parse(input);
    assert_eq!(result, Ok(22));
}

#[test]
fn test_parse_with_a_single_colon() {
    let input: &[u8] = b"localhost:";
    let result = parse(input);
    assert_eq!(result, Ok(10));
}

#[test]
fn test_parse_with_cut_off_at_sign() {
    let input: &[u8] = b"localhost@";
    let result = parse(input);
    assert_eq!(result, Ok(11));
}

#[test]
fn test_parse_invalid_case_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_case_end_bracket() {
    let input: &[u8] = b"[2001:db8::1";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_case_percent_sign() {
    let input: &[u8] = b"user:pass@localhost:80%";
    let result = parse(input);
    assert!(result.is_err());
}

