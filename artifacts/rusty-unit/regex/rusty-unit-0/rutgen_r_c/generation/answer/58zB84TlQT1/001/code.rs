// Answer 0

#[test]
fn test_new_byte_input_valid() {
    let text: &[u8] = b"Hello, world!";
    let only_utf8 = true;
    let input = ByteInput::new(text, only_utf8);
    assert_eq!(input.text, text);
    assert_eq!(input.only_utf8, only_utf8);
}

#[test]
fn test_new_byte_input_empty() {
    let text: &[u8] = b"";
    let only_utf8 = false;
    let input = ByteInput::new(text, only_utf8);
    assert_eq!(input.text, text);
    assert_eq!(input.only_utf8, only_utf8);
}

#[test]
fn test_new_byte_input_binary_data() {
    let text: &[u8] = &[0, 255, 127];
    let only_utf8 = false;
    let input = ByteInput::new(text, only_utf8);
    assert_eq!(input.text, text);
    assert_eq!(input.only_utf8, only_utf8);
}

#[test]
fn test_new_byte_input_utf8_only() {
    let text: &[u8] = "こんにちは".as_bytes();
    let only_utf8 = true;
    let input = ByteInput::new(text, only_utf8);
    assert_eq!(input.text, text);
    assert_eq!(input.only_utf8, only_utf8);
}

