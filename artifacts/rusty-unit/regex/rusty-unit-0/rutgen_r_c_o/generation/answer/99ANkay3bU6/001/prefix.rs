// Answer 0

#[test]
fn test_previous_char_empty_input() {
    let input = CharInput(&[]);
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    input.previous_char(at);
}

#[test]
fn test_previous_char_single_byte_ascii() {
    let input = CharInput(&[b'a']);
    let at = InputAt { pos: 1, c: Char(0), byte: Some(b'a'), len: 1 };
    input.previous_char(at);
}

#[test]
fn test_previous_char_two_bytes_utf8() {
    let input = CharInput(&[0xc2, 0xa9]); // É (Copyright sign in UTF-8)
    let at = InputAt { pos: 2, c: Char(0), byte: Some(0xa9), len: 2 };
    input.previous_char(at);
}

#[test]
fn test_previous_char_three_bytes_utf8() {
    let input = CharInput(&[0xe0, 0xa0, 0x80]); // U+0800 (3 bytes UTF-8)
    let at = InputAt { pos: 3, c: Char(0), byte: Some(0x80), len: 3 };
    input.previous_char(at);
}

#[test]
fn test_previous_char_four_bytes_utf8() {
    let input = CharInput(&[0xf0, 0x90, 0x80, 0x80]); // U+10000 (4 bytes UTF-8)
    let at = InputAt { pos: 4, c: Char(0), byte: Some(0x80), len: 4 };
    input.previous_char(at);
}

#[test]
fn test_previous_char_boundary_condition() {
    let input = CharInput(&[0x7f; 10]); // Boundary case: 10 ASCII max value
    let at = InputAt { pos: 10, c: Char(0), byte: Some(0x7f), len: 10 };
    input.previous_char(at);
}

#[test]
fn test_previous_char_insufficient_bytes() {
    let input = CharInput(&[0xe0, 0x80]); // Insufficient bytes for a 3-byte character
    let at = InputAt { pos: 2, c: Char(0), byte: Some(0x80), len: 2 };
    input.previous_char(at);
}

