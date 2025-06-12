// Answer 0

#[test]
fn test_next_char_valid_utf8() {
    let input_bytes: &[u8] = &[0b110_00010, 0b101_00001]; // Valid UTF-8 sequence for U+2011
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let input_at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };

    let result = input.next_char(input_at);
    assert_eq!(result, Char(0x2011)); // Expect Char for U+2011
}

#[test]
fn test_next_char_single_byte_utf8() {
    let input_bytes: &[u8] = &[0b01100001]; // Valid UTF-8 for 'a' (U+0061)
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let input_at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };

    let result = input.next_char(input_at);
    assert_eq!(result, Char(0x0061)); // Expect Char for 'a' (U+0061)
}

#[test]
fn test_next_char_invalid_utf8_sequence() {
    let input_bytes: &[u8] = &[0b11100000, 0b00000010]; // Invalid UTF-8 sequence
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let input_at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };

    let result = input.next_char(input_at);
    // Assuming `next_char` returns a default Char on invalid input
    assert_eq!(result, Char(0)); // Should return Char with 0 (or any defined invalid)
}

#[test]
fn test_next_char_out_of_bounds() {
    let input_bytes: &[u8] = &[0b110_00010, 0b101_00001];
    let input = ByteInput { text: input_bytes, only_utf8: true };

    // Simulate an input_at that is out of range
    let input_at = InputAt { pos: 3, c: Char(0), byte: None, len: 0 }; // pos is out of bounds

    // Should panic because of out-of-bounds access
    let result = std::panic::catch_unwind(|| {
        input.next_char(input_at);
    });

    assert!(result.is_err()); // Expect panic
}

