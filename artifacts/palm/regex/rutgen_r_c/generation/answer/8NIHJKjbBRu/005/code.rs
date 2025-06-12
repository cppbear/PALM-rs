// Answer 0

#[test]
fn test_next_utf8_out_of_bounds() {
    let text: &[u8] = b"hello"; // Example text
    let index = text.len(); // This is out of bounds
    let result = next_utf8(text, index);
    assert_eq!(result, index + 1); // Expect index + 1 when out of bounds
}

#[test]
fn test_next_utf8_valid_ascii() {
    let text: &[u8] = b"hello"; // Regular ASCII text
    let index = 0; // Starting with the first character
    let result = next_utf8(text, index);
    assert_eq!(result, index + 1); // Expect to move to the next character
}

#[test]
fn test_next_utf8_two_byte_character() {
    let text: &[u8] = &[0b1100_0010, 0b1010_0001, 0b0110_1010]; // Example of a two-byte character
    let index = 0; // Starting with the first byte of two-byte character
    let result = next_utf8(text, index);
    assert_eq!(result, index + 2); // Expect to skip to the next byte after the two-byte character
}

#[test]
fn test_next_utf8_three_byte_character() {
    let text: &[u8] = &[0b1110_0010, 0b1000_0001, 0b1000_0001]; // Example of a three-byte character
    let index = 0; // Starting with the first byte of three-byte character
    let result = next_utf8(text, index);
    assert_eq!(result, index + 3); // Expect to skip to the next byte after the three-byte character
}

#[test]
fn test_next_utf8_four_byte_character() {
    let text: &[u8] = &[0b1111_0010, 0b1000_0001, 0b1000_0001, 0b1000_0001]; // Example of a four-byte character
    let index = 0; // Starting with the first byte of the four-byte character
    let result = next_utf8(text, index);
    assert_eq!(result, index + 4); // Expect to skip to the next byte after the four-byte character
}

#[test]
fn test_next_utf8_multiple_characters() {
    let text: &[u8] = b"hello \xe2\x9c\x94"; // ASCII followed by a multi-byte character (checkmark)
    let index = 6; // The index right after the ASCII characters
    let result = next_utf8(text, index);
    assert_eq!(result, index + 3); // Expect to skip past the multi-byte character
}

