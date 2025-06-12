// Answer 0

#[test]
fn test_new_char_input() {
    let input_bytes: &[u8] = b"hello";
    let char_input = CharInput::new(input_bytes);
    assert_eq!(char_input.0, input_bytes);
}

#[test]
fn test_new_char_input_empty() {
    let input_bytes: &[u8] = b"";
    let char_input = CharInput::new(input_bytes);
    assert_eq!(char_input.0, input_bytes);
}

#[test]
fn test_new_char_input_unicode() {
    let input_bytes: &[u8] = "こんにちは".as_bytes();
    let char_input = CharInput::new(input_bytes);
    assert_eq!(char_input.0, input_bytes);
}

