// Answer 0

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let src = vec![0b11111111]; // Invalid first byte, should return None
    assert_eq!(decode_utf8(&src), None);
}

#[test]
fn test_decode_utf8_incomplete_sequence_two_bytes() {
    let src = vec![0b11000010]; // Valid start for two bytes but incomplete
    assert_eq!(decode_utf8(&src), None);
}

#[test]
fn test_decode_utf8_incomplete_sequence_three_bytes() {
    let src = vec![0b11100010]; // Valid start for three bytes but incomplete
    assert_eq!(decode_utf8(&src), None);
}

#[test]
fn test_decode_utf8_incomplete_sequence_four_bytes() {
    let src = vec![0b11110010]; // Valid start for four bytes but incomplete
    assert_eq!(decode_utf8(&src), None);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte() {
    let src = vec![0b11000010, 0b00000000]; // Invalid continuation byte
    assert_eq!(decode_utf8(&src), None);
}

#[test]
fn test_decode_utf8_surrogate_codepoint() {
    let src = vec![0b11110000, 0b10111111, 0b10111111, 0b10111111]; // Surrogate codepoint, should return None
    assert_eq!(decode_utf8(&src), None);
}

#[test]
fn test_decode_utf8_empty_input() {
    let src: Vec<u8> = vec![]; // Empty input, should return None
    assert_eq!(decode_utf8(&src), None);
}

