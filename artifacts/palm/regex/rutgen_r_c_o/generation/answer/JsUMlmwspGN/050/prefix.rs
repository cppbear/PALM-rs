// Answer 0

#[test]
fn test_decode_utf8_four_byte_sequence() {
    let src: [u8; 4] = [0b11110_000, 0b10000000, 0b10000000, 0b10000000];
    decode_utf8(&src);
}

#[test]
fn test_decode_utf8_four_byte_sequence_max() {
    let src: [u8; 4] = [0b11110_111, 0b10111111, 0b10111111, 0b10111111];
    decode_utf8(&src);
}

#[test]
fn test_decode_utf8_four_byte_sequence_middle() {
    let src: [u8; 4] = [0b11110_100, 0b10000000, 0b10000000, 0b10000000];
    decode_utf8(&src);
}

#[test]
fn test_decode_utf8_four_byte_sequence_combination() {
    let src: [u8; 4] = [0b11110_101, 0b10110000, 0b10101000, 0b10000000];
    decode_utf8(&src);
}

#[test]
fn test_decode_utf8_four_byte_surrogate_codepoint() {
    let src: [u8; 4] = [0b11110_101, 0b10111000, 0b10111100, 0b10000000];
    decode_utf8(&src);
}

