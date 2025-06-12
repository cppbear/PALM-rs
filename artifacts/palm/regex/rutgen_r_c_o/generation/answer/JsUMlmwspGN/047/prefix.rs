// Answer 0

#[test]
fn test_decode_utf8_invalid_four_bytes() {
    let input: &[u8] = &[0b11110_000, 0b00_000000, 0b10_000000, 0b10_000000];
    let result = decode_utf8(input);
}

#[test]
fn test_decode_utf8_invalid_four_bytes_other() {
    let input: &[u8] = &[0b11110_001, 0b00_000000, 0b10_000000, 0b10_000000];
    let result = decode_utf8(input);
}

#[test]
fn test_decode_utf8_invalid_four_bytes_boundary() {
    let input: &[u8] = &[0b11110_111, 0b00_000000, 0b10_000000, 0b10_000000];
    let result = decode_utf8(input);
}

