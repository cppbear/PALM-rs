// Answer 0

#[test]
fn test_decode_utf8_invalid_four_byte_sequence() {
    let src = [0b11110_000, 0b1011_1111, 0b0011_1111, 0b1010_0000];
    decode_utf8(&src);
}

#[test]
fn test_decode_utf8_invalid_four_byte_sequence_short() {
    let src = [0b11110_000, 0b1011_1111, 0b0000_1111];
    decode_utf8(&src);
}

#[test]
fn test_decode_utf8_valid_four_byte_sequence() {
    let src = [0b11110_000, 0b1011_1111, 0b1011_1111, 0b1011_1111];
    decode_utf8(&src);
}

