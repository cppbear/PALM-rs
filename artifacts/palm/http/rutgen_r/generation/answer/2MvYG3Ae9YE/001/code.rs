// Answer 0

#[test]
fn test_from_lowercase_parses_valid_lowercase_header() {
    let hdr = from_lowercase(b"content-length").unwrap();
    assert_eq!(CONTENT_LENGTH, hdr);
}

#[test]
fn test_from_lowercase_fails_on_uppercase_header() {
    assert!(from_lowercase(b"Content-Length").is_err());
}

#[test]
fn test_from_lowercase_fails_on_invalid_characters() {
    assert!(from_lowercase(b"content-length\xFF").is_err());
}

#[test]
fn test_from_lowercase_empty_input() {
    assert!(from_lowercase(b"").is_err());
}

#[test]
fn test_from_lowercase_mixed_case_input() {
    assert!(from_lowercase(b"CoNtEnT-LenGth").is_err());
}

#[test]
fn test_from_lowercase_invalid_utf8_bytes() {
    assert!(from_lowercase(b"cont\xFFent-length").is_err());
}

#[test]
fn test_from_lowercase_whitespace() {
    assert!(from_lowercase(b"content length").is_err());
}

