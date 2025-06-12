// Answer 0

#[test]
fn test_extension_inline_success() {
    let input: &[u8] = b"ExampleMethod"; // Length 13, which is valid
    let result = Method::extension_inline(input);
    assert!(result.is_ok());
}

#[test]
fn test_extension_inline_too_short() {
    let input: &[u8] = b""; // Empty input
    let result = Method::extension_inline(input);
    assert!(result.is_err());
}

#[test]
fn test_extension_inline_too_long() {
    let input: &[u8] = b"TooLongMethodString"; // Length 18, exceeds MAX
    let result = Method::extension_inline(input);
    assert!(result.is_err());
}

#[test]
fn test_extension_inline_edge_case_max_length() {
    let input: &[u8] = b"MaxLengthMethod"; // Length 15, exactly MAX
    let result = Method::extension_inline(input);
    assert!(result.is_ok());
}

#[test]
fn test_extension_inline_invalid_characters() {
    let input: &[u8] = &[255]; // Invalid byte that might not be accepted
    let result = Method::extension_inline(input);
    assert!(result.is_err());
}

