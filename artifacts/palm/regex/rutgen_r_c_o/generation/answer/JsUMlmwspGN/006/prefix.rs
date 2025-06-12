// Answer 0

#[test]
fn test_decode_utf8_valid_two_byte_sequence() {
    let input = [0b110_00001, 0b10000000]; 
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_invalid_two_byte_sequence() {
    let input = [0b110_00000, 0b10000001];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_too_short_two_byte_sequence() {
    let input = [0b110_00000];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let input = [0b111_11111, 0b10000000];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_complete_case_two_byte_valid() {
    let input = [0b110_00010, 0b10000000]; // a valid two-byte sequence
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_not_utf8_sequence() {
    let input = [0xFF, 0xFF]; // invalid byte sequence
    decode_utf8(&input);
}

