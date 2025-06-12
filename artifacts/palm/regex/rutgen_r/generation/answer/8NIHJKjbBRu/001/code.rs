// Answer 0

#[test]
fn test_next_utf8_valid_ascii() {
    let text: &[u8] = b"Hello, world!";
    let start_index = 0;
    let result = next_utf8(text, start_index);
    assert_eq!(result, start_index + 1); // Expect moving to next byte
}

#[test]
fn test_next_utf8_valid_two_byte_sequence() {
    let text: &[u8] = b"\xC2\xA9"; // © character
    let start_index = 0;
    let result = next_utf8(text, start_index);
    assert_eq!(result, start_index + 2); // Expect moving past two bytes
}

#[test]
fn test_next_utf8_valid_three_byte_sequence() {
    let text: &[u8] = b"\xE2\x82\xAC"; // € character
    let start_index = 0;
    let result = next_utf8(text, start_index);
    assert_eq!(result, start_index + 3); // Expect moving past three bytes
}

#[test]
fn test_next_utf8_valid_four_byte_sequence() {
    let text: &[u8] = b"\xF0\x9F\x98\x80"; // 😀 character
    let start_index = 0;
    let result = next_utf8(text, start_index);
    assert_eq!(result, start_index + 4); // Expect moving past four bytes
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text: &[u8] = b"";
    let start_index = 0;
    let result = next_utf8(text, start_index);
    assert_eq!(result, start_index + 1); // Out of bounds; expect moving to next index
}

#[test]
fn test_next_utf8_at_ascii_boundary() {
    let text: &[u8] = b"\x7F"; // Boundary value for single byte
    let start_index = 0;
    let result = next_utf8(text, start_index);
    assert_eq!(result, start_index + 1); // Expect moving to next index
}

