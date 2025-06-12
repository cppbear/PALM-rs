// Answer 0

#[test]
fn test_parse_valid_authority_with_single_colon() {
    let input: &[u8] = b"localhost:8080/path";
    let result = http::parse(input);
    assert_eq!(result, Ok(17)); // "localhost:8080" ends at index 17
}

#[test]
fn test_parse_valid_authority_with_ipv6() {
    let input: &[u8] = b"[::1]:80";
    let result = http::parse(input);
    assert_eq!(result, Ok(8)); // "[::1]" ends at index 8
}

#[test]
fn test_parse_valid_authority_with_userinfo() {
    let input: &[u8] = b"user:password@localhost:8080";
    let result = http::parse(input);
    assert_eq!(result, Ok(29)); // "user:password@localhost" ends at index 29
}

#[test]
fn test_parse_valid_authority_with_escape_sequences() {
    let input: &[u8] = b"example.com:80/path";
    let result = http::parse(input);
    assert_eq!(result, Ok(17)); // "example.com:80" ends at index 17
}

#[test]
fn test_parse_valid_authority_end_with_question_mark() {
    let input: &[u8] = b"localhost:8080?query=string";
    let result = http::parse(input);
    assert_eq!(result, Ok(17)); // "localhost:8080" ends at index 17
}

#[test]
fn test_parse_valid_authority_with_brackets() {
    let input: &[u8] = b"[2001:db8::1]:12345";
    let result = http::parse(input);
    assert_eq!(result, Ok(18)); // "[2001:db8::1]" ends at index 18
}

