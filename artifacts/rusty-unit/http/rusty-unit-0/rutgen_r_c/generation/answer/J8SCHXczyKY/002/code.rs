// Answer 0

#[test]
fn test_extension_inline_valid_input() {
    // Test with valid input that fits the constraints of InlineExtension::new
    let input: &[u8] = b"GET / HTTP/1.1"; // A valid input string within the size limit
    let result = Method::extension_inline(input);

    // Check that the result is Ok
    assert!(result.is_ok());

    // Check that the returned Method is of the correct variant
    match result {
        Ok(method) => match method.0 {
            Inner::ExtensionInline(_) => {}
            _ => panic!("Expected Inner::ExtensionInline"),
        },
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_extension_inline_exact_length() {
    // Test with input that hits the maximum length boundary
    let input: &[u8] = b"abcdefghijklmno"; // 15 bytes exactly
    let result = Method::extension_inline(input);

    // Check that the result is Ok
    assert!(result.is_ok());

    // Check for the proper variant
    match result {
        Ok(method) => match method.0 {
            Inner::ExtensionInline(_) => {}
            _ => panic!("Expected Inner::ExtensionInline"),
        },
        _ => panic!("Expected Ok result"),
    }
}

#[test]
#[should_panic]
fn test_extension_inline_too_long_input() {
    // Test with input exceeding the maximum length
    let input: &[u8] = b"abcdefghijklmnop"; // 16 bytes, should panic
    let _ = Method::extension_inline(input);
}

#[test]
fn test_extension_inline_empty_input() {
    // Test with empty input
    let input: &[u8] = b""; // Empty input is valid
    let result = Method::extension_inline(input);

    // Check that the result is Ok
    assert!(result.is_ok());

    // Check for the proper variant
    match result {
        Ok(method) => match method.0 {
            Inner::ExtensionInline(_) => {}
            _ => panic!("Expected Inner::ExtensionInline"),
        },
        _ => panic!("Expected Ok result"),
    }
}

