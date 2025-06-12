// Answer 0

#[test]
fn test_from_bytes_empty_input() {
    let result = HdrName::from_bytes(b"", |name| name);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_characters() {
    let invalid_bytes = b"Invalid\x80Header";
    let result = HdrName::from_bytes(invalid_bytes, |name| name);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_custom_header() {
    let valid_bytes = b"Custom-Header";
    let result = HdrName::from_bytes(valid_bytes, |name| name);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_overflow_input() {
    let overflow_bytes = b"This header name is way too long for normal usage and should trigger the overflow handling in the function's implementation which means it should not be parsed as a standard header.";
    let result = HdrName::from_bytes(overflow_bytes, |name| name);
    assert!(result.is_ok());
}

#[test]
fn test_from_bytes_contain_null_character() {
    let bytes_with_null = b"Header\x00Name";
    let result = HdrName::from_bytes(bytes_with_null, |name| name);
    assert!(result.is_err());
}

