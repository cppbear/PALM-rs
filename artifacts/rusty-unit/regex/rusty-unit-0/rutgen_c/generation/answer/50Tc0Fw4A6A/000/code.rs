// Answer 0

#[test]
fn test_next_char_valid_utf8() {
    let input_bytes: &[u8] = b"hello";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    
    let result = input.next_char(at);
    assert_eq!(result, Char('h' as u32));
}

#[test]
fn test_next_char_invalid_utf8() {
    let input_bytes: &[u8] = b"\xFF"; // Invalid UTF-8 byte
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    
    let result = input.next_char(at);
    assert_eq!(result, Char(0)); // Assuming Char(0) represents an invalid character
}

#[test]
fn test_next_char_multiple_bytes() {
    let input_bytes: &[u8] = b"\xE2\x9C\x94"; // UTF-8 for a check mark
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    
    let result = input.next_char(at);
    assert_eq!(result, Char(0x2714)); // Unicode code point for check mark
}

#[test]
fn test_next_char_overflow() {
    let input_bytes: &[u8] = b"hello";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let at = InputAt { pos: 5, c: Char(0), byte: None, len: 0 }; // Position at end of string

    let result = input.next_char(at);
    assert_eq!(result, Char(0)); // Assuming Char(0) represents an invalid character
}

