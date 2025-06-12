// Answer 0

#[test]
fn test_new_char_input_non_empty() {
    let input = b"Hello, world!";
    let char_input = CharInput::new(input);
    assert_eq!(char_input.0, input);
}

#[test]
fn test_new_char_input_empty() {
    let input: &[u8] = b"";
    let char_input = CharInput::new(input);
    assert_eq!(char_input.0, input);
}

#[test]
fn test_new_char_input_with_unicode() {
    let input = "こんにちは".as_bytes();
    let char_input = CharInput::new(input);
    assert_eq!(char_input.0, input);
}

#[test]
fn test_new_char_input_with_special_chars() {
    let input = b"@#$%^&*()_+|";
    let char_input = CharInput::new(input);
    assert_eq!(char_input.0, input);
}

