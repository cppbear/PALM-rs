// Answer 0

#[test]
fn test_new_valid_input() {
    let input: &[u8] = b"valid input";
    let result = http::new(input);
    assert!(result.is_ok());
    let inline_extension = result.unwrap();
    assert_eq!(inline_extension.1, input.len() as u8);
}

#[test]
fn test_new_empty_input() {
    let input: &[u8] = b"";
    let result = http::new(input);
    assert!(result.is_ok());
    let inline_extension = result.unwrap();
    assert_eq!(inline_extension.1, 0);
}

#[test]
fn test_new_too_long_input() {
    let input: &[u8] = &[b'a';http::InlineExtension::MAX as usize + 1]; // Input exceeding MAX length
    let result = http::new(input);
    assert!(result.is_err());
}

#[test]
fn test_new_invalid_utf8_input() {
    let input: &[u8] = &[0xff, 0xfe]; // Invalid UTF-8 sequence
    let result = http::new(input);
    assert!(result.is_err());
}

