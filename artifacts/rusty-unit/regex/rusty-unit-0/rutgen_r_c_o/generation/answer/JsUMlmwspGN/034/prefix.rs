// Answer 0

#[test]
fn test_decode_utf8_3_byte_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b1000_0000, 0b1000_0000];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_3_byte_sequence_alternative() {
    let input: &[u8] = &[0b1110_1111, 0b1011_1111, 0b1011_1111];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_3_byte_sequence_mid_range() {
    let input: &[u8] = &[0b1110_1000, 0b1010_1010, 0b1000_0000];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_3_byte_sequence_with_high_unicode() {
    let input: &[u8] = &[0b1110_1001, 0b1010_1111, 0b1000_0001];
    decode_utf8(input);
}

