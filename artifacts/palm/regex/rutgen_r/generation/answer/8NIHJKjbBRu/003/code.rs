// Answer 0

#[test]
fn test_next_utf8_valid_sequence() {
    let text = &[0b1110_1111, 0b10_000000]; // Valid UTF-8 sequence starting with 0b1110_1111
    let index = 0; // starting index
    let result = next_utf8(text, index);
    assert_eq!(result, 2); // Expecting index 0 + inc (3), which would point to the next byte
}

#[test]
fn test_next_utf8_invalid_sequence() {
    let text = &[0b110_11111, 0b10_000000]; // Invalid UTF-8 sequence starting with 0b110_11111
    let index = 0; // starting index
    let result = next_utf8(text, index);
    assert_eq!(result, 1); // Expecting index 0 + inc (2), which points to next byte
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text = &[0u8]; // Single byte
    let index = 1; // starting index out of bounds
    let result = next_utf8(text, index);
    assert_eq!(result, 2); // Expecting index 1 + 1
}

#[test]
fn test_next_utf8_boundary_valid() {
    let text = &[0b1110_1111]; // Single valid byte that forms a start of a sequence
    let index = 0; // starting index
    let result = next_utf8(text, index);
    assert_eq!(result, 1); // Expecting index 0 + inc (1)
}

