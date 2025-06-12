// Answer 0

#[test]
fn test_new_byte_input() {
    let text: &[u8] = b"Hello, World!";
    let only_utf8 = true;

    let byte_input = ByteInput::new(text, only_utf8);

    assert_eq!(byte_input.text, text);
    assert_eq!(byte_input.only_utf8, only_utf8);
}

#[test]
fn test_new_byte_input_empty() {
    let text: &[u8] = b"";
    let only_utf8 = false;

    let byte_input = ByteInput::new(text, only_utf8);

    assert_eq!(byte_input.text, text);
    assert_eq!(byte_input.only_utf8, only_utf8);
}

#[test]
fn test_new_byte_input_non_utf8() {
    let text: &[u8] = &[0x80, 0x81, 0x82]; // Non-UTF-8 bytes
    let only_utf8 = false;

    let byte_input = ByteInput::new(text, only_utf8);

    assert_eq!(byte_input.text, text);
    assert_eq!(byte_input.only_utf8, only_utf8);
}

