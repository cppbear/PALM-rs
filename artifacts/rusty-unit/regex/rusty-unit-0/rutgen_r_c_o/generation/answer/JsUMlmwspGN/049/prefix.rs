// Answer 0

#[test]
fn test_decode_utf8_four_byte_invalid() {
    let src: &[u8] = &[0b11110_000, 0b10_000000, 0b10_000000, 0b00_000000]; // b3 is invalid
    decode_utf8(src);
}

#[test]
fn test_decode_utf8_four_byte_invalid_two() {
    let src: &[u8] = &[0b11110_001, 0b10_000000, 0b10_000000, 0b00_000000]; // b3 is invalid
    decode_utf8(src);
}

#[test]
fn test_decode_utf8_four_byte_invalid_three() {
    let src: &[u8] = &[0b11110_010, 0b10_000000, 0b10_000000, 0b00_000000]; // b3 is invalid
    decode_utf8(src);
}

#[test]
fn test_decode_utf8_four_byte_invalid_four() {
    let src: &[u8] = &[0b11110_111, 0b10_000000, 0b10_000000, 0b00_000000]; // b3 is invalid
    decode_utf8(src);
}

