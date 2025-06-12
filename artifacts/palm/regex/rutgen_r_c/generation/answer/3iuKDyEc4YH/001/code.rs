// Answer 0

#[test]
fn test_previous_char_valid() {
    let text: &[u8] = &[0xE2, 0x9C, 0x94]; // Unicode character '✔'
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 3, c: Char(0), byte: None, len: 1 };
    let result = input.previous_char(at);
    assert_eq!(result.0, 0x2705); // Check that it returns the correct UTF-8 decoded character
}

#[test]
#[should_panic]
fn test_previous_char_empty() {
    let text: &[u8] = &[]; // Empty byte array
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 }; // Position is 0
    input.previous_char(at); // This should panic due to out-of-bounds access
}

#[test]
fn test_previous_char_single_byte() {
    let text: &[u8] = &[0x41]; // ASCII character 'A'
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };
    let result = input.previous_char(at);
    assert_eq!(result.0, 0x41); // Check that it returns the correct character
}

#[test]
#[should_panic]
fn test_previous_char_inaccessible_position() {
    let text: &[u8] = &[0xE2, 0x9C, 0x94]; // Unicode example character
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 4, c: Char(0), byte: None, len: 1 }; // Position out of bounds
    input.previous_char(at); // This should panic
}

