// Answer 0

#[test]
fn test_previous_char_start_of_input() {
    let input_data: &[u8] = b"abc";
    let input = ByteInput { text: input_data, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    
    let result = input.previous_char(at);
    assert_eq!(result.0, 0); // Expecting the char value corresponding to the start of the input
}

#[test]
fn test_previous_char_single_byte_utf8() {
    let input_data: &[u8] = b"a";
    let input = ByteInput { text: input_data, only_utf8: true };
    let at = InputAt { pos: 1, c: Char(0), byte: None, len: 0 };
    
    let result = input.previous_char(at);
    assert_eq!(result.0, 'a' as u32); // Expecting the char value for 'a'
}

#[test]
fn test_previous_char_multi_byte_utf8() {
    let input_data: &[u8] = b"\xE2\x9C\x94"; // U+2714 CHECK MARK
    let input = ByteInput { text: input_data, only_utf8: true };
    let at = InputAt { pos: 3, c: Char(0), byte: None, len: 0 };
    
    let result = input.previous_char(at);
    assert_eq!(result.0, 0x2714); // Expecting the char value for the check mark
}

#[test]
fn test_previous_char_empty_input() {
    let input_data: &[u8] = b"";
    let input = ByteInput { text: input_data, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    
    let result = input.previous_char(at);
    // If pos is 0, we expect it to handle gracefully, potentially returning a default or invalid char.
    assert_eq!(result.0, 0); // Assuming it returns 0 for empty input scenario
}

