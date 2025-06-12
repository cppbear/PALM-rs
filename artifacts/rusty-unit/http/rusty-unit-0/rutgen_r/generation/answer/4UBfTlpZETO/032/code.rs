// Answer 0

#[test]
fn test_parse_invalid_authority_too_many_colons() {
    let input: &[u8] = b"localhost:8080:3030";
    let result = http::parse(input);
    assert_eq!(result, Err(http::ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_unmatched_brackets() {
    let input: &[u8] = b"[2001:db8::ff00:42:8329:80"; // Missing closing bracket
    let result = http::parse(input);
    assert_eq!(result, Err(http::ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_percent_character_after_userinfo() {
    let input: &[u8] = b"user:pass@localhost%:80";
    let result = http::parse(input);
    assert_eq!(result, Err(http::ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_invalid_uri_char() {
    let input: &[u8] = b"localhost\x00:80"; // Invalid URI character (null byte)
    let result = http::parse(input);
    assert_eq!(result, Err(http::ErrorKind::InvalidUriChar.into()));
}

#[test]
fn test_parse_invalid_authority_mismatched_start_end_brackets() {
    let input: &[u8] = b"[2001:db8::ff00:42:8329:80]";
    let result = http::parse(input);
    assert_eq!(result, Err(http::ErrorKind::InvalidAuthority.into()));
}

#[test]
fn test_parse_invalid_authority_two_percent_encodings() {
    let input: &[u8] = b"user@foo%:80"; // Invalid due to percent encoding after userinfo
    let result = http::parse(input);
    assert_eq!(result, Err(http::ErrorKind::InvalidAuthority.into()));
}

