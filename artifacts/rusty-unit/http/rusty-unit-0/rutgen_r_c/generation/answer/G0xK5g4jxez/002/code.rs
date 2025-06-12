// Answer 0

#[test]
fn test_new_valid_input() {
    let valid_input = b"GET";
    let result = InlineExtension::new(valid_input);
    assert!(result.is_ok());

    if let Ok(extension) = result {
        assert_eq!(extension.1, valid_input.len() as u8);
    }
}

#[test]
fn test_new_valid_input_boundary() {
    let valid_input = b"GET /index.html";
    let result = InlineExtension::new(valid_input);
    assert!(result.is_ok());

    if let Ok(extension) = result {
        assert_eq!(extension.1, valid_input.len() as u8);
    }
}

#[test]
fn test_new_invalid_input() {
    let invalid_input = b"\xFF"; // Invalid byte
    let result = InlineExtension::new(invalid_input);
    assert!(result.is_err());
}

#[test]
fn test_new_empty_input() {
    let empty_input = b"";
    let result = InlineExtension::new(empty_input);
    assert!(result.is_ok());

    if let Ok(extension) = result {
        assert_eq!(extension.1, empty_input.len() as u8);
    }
}

#[test]
fn test_new_too_long_input() {
    let long_input = b"GET /index.html?param=value&other=value"; // Length exceeds MAX
    let result = InlineExtension::new(long_input);
    assert!(result.is_err());
}

