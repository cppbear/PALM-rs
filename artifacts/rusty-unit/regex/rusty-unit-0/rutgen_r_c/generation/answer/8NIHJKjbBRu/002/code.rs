// Answer 0

#[test]
fn test_next_utf8_valid_two_byte_sequence() {
    let text: &[u8] = &[0b110_11111]; // b <= 0b110_11111 is true, with bound b == 0b110_11111
    let i = 0; // valid index
    let result = next_utf8(text, i);
    assert_eq!(result, 2); // i + inc should equal 2 (0 + 2)
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text: &[u8] = &[0b110_11111]; // only one byte in the array
    let i = 1; // out of bounds
    let result = next_utf8(text, i);
    assert_eq!(result, 2); // i + 1 should equal 2 (1 + 1)
}

#[test]
fn test_next_utf8_valid_two_byte_sequence_with_padding() {
    let text: &[u8] = &[0b110_11111, 0xFF]; // two valid bytes in UTF-8
    let i = 0; // valid index
    let result = next_utf8(text, i);
    assert_eq!(result, 2); // i + inc should equal 2 (0 + 2)
}

#[test]
fn test_next_utf8_first_byte_three_byte_sequence() {
    let text: &[u8] = &[0b1110_1111]; // valid three-byte starting byte
    let i = 0; // valid index
    let result = next_utf8(text, i);
    assert_eq!(result, 4); // i + inc should equal 4 (0 + 4)
}

#[test]
fn test_next_utf8_second_byte_three_byte_sequence() {
    let text: &[u8] = &[0b1110_1111, 0b1000_0000]; // valid three-byte sequence
    let i = 1; // valid index for second byte
    let result = next_utf8(text, i);
    assert_eq!(result, 4); // i + inc should equal 4 (1 + 3)
}

