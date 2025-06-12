// Answer 0

#[test]
fn test_new_valid_method() {
    let src = b"GET";
    let result = InlineExtension::new(src);
    assert!(result.is_ok());
    let extension = result.unwrap();
    assert_eq!(extension.1, 3); // Length should match the input
}

#[test]
fn test_new_invalid_method() {
    let src = b"INVALID\xFF"; // Contains an invalid byte
    let result = InlineExtension::new(src);
    assert!(result.is_err());
}

#[test]
fn test_new_empty_method() {
    let src: &[u8] = b""; // Test with empty input
    let result = InlineExtension::new(src);
    assert!(result.is_ok());
    let extension = result.unwrap();
    assert_eq!(extension.1, 0); // Length should be 0
}

#[test]
fn test_new_method_too_long() {
    let src = b"GET GET GET"; // Input length exceeds INLINE_EXTENSION_MAX
    let result = InlineExtension::new(src);
    assert!(result.is_err());
}

#[test]
fn test_new_invalid_character() {
    let src = b"POST\x80"; // Invalid byte (0x80)
    let result = InlineExtension::new(src);
    assert!(result.is_err());
}

