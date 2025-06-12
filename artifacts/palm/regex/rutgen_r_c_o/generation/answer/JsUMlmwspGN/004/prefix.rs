// Answer 0

#[test]
fn test_decode_utf8_valid_two_byte_sequence_1() {
    let input = [0xC0, 0xA0];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_valid_two_byte_sequence_2() {
    let input = [0xC1, 0x80];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_valid_two_byte_sequence_3() {
    let input = [0xC3, 0xBF];
    decode_utf8(&input);
}

