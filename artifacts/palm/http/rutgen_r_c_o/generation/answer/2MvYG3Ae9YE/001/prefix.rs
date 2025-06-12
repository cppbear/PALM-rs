// Answer 0

#[test]
fn test_from_lowercase_valid_short() {
    let result = HeaderName::from_lowercase(b"a");
}

#[test]
fn test_from_lowercase_valid_medium() {
    let result = HeaderName::from_lowercase(b"accept-charset");
}

#[test]
fn test_from_lowercase_valid_long() {
    let result = HeaderName::from_lowercase(b"content-length");
}

#[test]
fn test_from_lowercase_invalid_empty() {
    let result = HeaderName::from_lowercase(b"");
}

#[test]
fn test_from_lowercase_invalid_uppercase() {
    let result = HeaderName::from_lowercase(b"Content-Length");
}

#[test]
fn test_from_lowercase_invalid_special_chars() {
    let result = HeaderName::from_lowercase(b"content!length");
}

#[test]
fn test_from_lowercase_valid_boundary() {
    let long_input = vec![b'a'; SCRATCH_BUF_SIZE];
    let result = HeaderName::from_lowercase(&long_input);
}

#[test]
fn test_from_lowercase_overflow() {
    let overflow_input = vec![b'a'; SCRATCH_BUF_OVERFLOW];
    let result = HeaderName::from_lowercase(&overflow_input);
}

#[test]
fn test_from_lowercase_invalid_non_lowercase() {
    let result = HeaderName::from_lowercase(b"content-length-123");
}

#[test]
fn test_from_lowercase_invalid_non_ascii() {
    let result = HeaderName::from_lowercase(b"\xFF\xFE");
}

