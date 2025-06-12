// Answer 0

#[test]
fn test_new_valid_method() {
    let valid_method: &[u8] = b"GET";
    let extension = InlineExtension::new(valid_method).expect("Expected to create InlineExtension");
    assert_eq!(extension.1, 3);
}

#[test]
fn test_new_valid_method_max_length() {
    let valid_method: &[u8] = b"GET/POST";
    let extension = InlineExtension::new(valid_method).expect("Expected to create InlineExtension");
    assert_eq!(extension.1, 8);
}

#[test]
fn test_new_invalid_method() {
    // Invalid because the byte value b'\x80' is outside the valid range
    let invalid_method: &[u8] = b"\x80INVALID";
    assert!(InlineExtension::new(invalid_method).is_err());
}

#[test]
fn test_new_invalid_method_with_null_bytes() {
    let invalid_method: &[u8] = b"GET\0INVALID";
    assert!(InlineExtension::new(invalid_method).is_err());
}

#[test]
fn test_new_empty_method() {
    let empty_method: &[u8] = b"";
    let extension = InlineExtension::new(empty_method).expect("Expected to create InlineExtension");
    assert_eq!(extension.1, 0);
}

#[test]
fn test_new_over_max_length() {
    let over_max_length: &[u8] = b"GET/POST/TOO/LONG";
    assert!(InlineExtension::new(over_max_length).is_err());
}

