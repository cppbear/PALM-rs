// Answer 0

#[test]
fn test_previous_char_valid_input() {
    let input = CharInput(b"Hello, world!");
    let at = InputAt { pos: 5, c: Char(0), byte: None, len: 0 };
    let result = input.previous_char(at);
    assert_eq!(result.0, 'o' as u32); // 'o' is the character before the position 5
}

#[test]
fn test_previous_char_empty_input() {
    let input = CharInput(b"");
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    let result = input.previous_char(at);
    assert_eq!(result.0, 0 as u32); // Assuming an empty input returns a null character
}

#[test]
fn test_previous_char_single_byte_input() {
    let input = CharInput(b"A");
    let at = InputAt { pos: 1, c: Char(0), byte: None, len: 0 };
    let result = input.previous_char(at);
    assert_eq!(result.0, 'A' as u32); // 'A' is the only character
}

#[test]
fn test_previous_char_multibyte_character() {
    let input = CharInput(b"\xE2\x9C\x93"); // ✓ character (U+2713)
    let at = InputAt { pos: 3, c: Char(0), byte: None, len: 0 };
    let result = input.previous_char(at);
    assert_eq!(result.0, '✓' as u32); // ✓ is the character preceding the position 3
}

