// Answer 0

#[test]
fn test_next_utf8_valid_four_byte_sequence() {
    let text = &[0xF0, 0x90, 0x80, 0x80]; // UTF-8 sequence for U+10000
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 4); // Next valid UTF-8 index after the four-byte sequence
}

#[test]
fn test_next_utf8_valid_three_byte_sequence() {
    let text = &[0xE0, 0xA0, 0x80]; // UTF-8 sequence for U+0800
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 3); // Next valid UTF-8 index after the three-byte sequence
}

#[test]
fn test_next_utf8_valid_two_byte_sequence() {
    let text = &[0xC2, 0xA9]; // UTF-8 sequence for U+00A9
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 2); // Next valid UTF-8 index after the two-byte sequence
}

#[test]
fn test_next_utf8_single_byte() {
    let text = &[0x80]; // Invalid single byte
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 1); // Since b <= 0x7F is false, we should skip it and return i + 1
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text: &[u8] = &[]; // Empty input
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 1); // Since `text.get(i)` is None, return i + 1
}

