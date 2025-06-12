// Answer 0

#[test]
fn test_decode_utf8_case_empty() {
    assert_eq!(decode_utf8(&[]), None);
}

#[test]
fn test_decode_utf8_case_valid_ascii() {
    assert_eq!(decode_utf8(&[0x41]), Some(('A', 1)));
}

#[test]
fn test_decode_utf8_case_invalid_two_byte_sequence_too_short() {
    assert_eq!(decode_utf8(&[0b11000000]), None);
}

#[test]
fn test_decode_utf8_case_invalid_two_byte_sequence_range_check() {
    assert_eq!(decode_utf8(&[0b11000000, 0b00000000]), Some(('\u{00}', 2)));
}

#[test]
fn test_decode_utf8_case_valid_two_byte_sequence() {
    assert_eq!(decode_utf8(&[0b11000010, 0b10000010]), Some(('Ђ', 2)));
}

#[test]
fn test_decode_utf8_case_invalid_two_byte_sequence_continuation_check() {
    assert_eq!(decode_utf8(&[0b11000010, 0b11111111]), None);
}

#[test]
fn test_decode_utf8_case_invalid_three_byte_sequence_too_short() {
    assert_eq!(decode_utf8(&[0b11100000, 0b10000000]), None);
}

#[test]
fn test_decode_utf8_case_invalid_three_byte_sequence_range_check() {
    assert_eq!(decode_utf8(&[0b11100000, 0b10000000, 0b00000000]), Some(('\u{00}', 3)));
}

#[test]
fn test_decode_utf8_case_valid_three_byte_sequence() {
    assert_eq!(decode_utf8(&[0b11100010, 0b10000010, 0b10000100]), Some(('â', 3)));
}

#[test]
fn test_decode_utf8_case_invalid_three_byte_sequence_continuation_check() {
    assert_eq!(decode_utf8(&[0b11100010, 0xFF, 0b10000100]), None);
}

#[test]
fn test_decode_utf8_case_invalid_four_byte_sequence_too_short() {
    assert_eq!(decode_utf8(&[0b11110000, 0b10000000, 0b10000000]), None);
}

#[test]
fn test_decode_utf8_case_invalid_four_byte_sequence_range_check() {
    assert_eq!(decode_utf8(&[0b11110000, 0b10000000, 0b10000000, 0b00000000]), Some(('\u{00}', 4)));
}

#[test]
fn test_decode_utf8_case_valid_four_byte_sequence() {
    assert_eq!(decode_utf8(&[0b11110000, 0b10010000, 0b10000000, 0b10000000]), Some(('𐀀', 4)));
}

#[test]
fn test_decode_utf8_case_invalid_four_byte_sequence_continuation_check() {
    assert_eq!(decode_utf8(&[0b11110000, 0xFF, 0b10000000, 0b10000000]), None);
}

