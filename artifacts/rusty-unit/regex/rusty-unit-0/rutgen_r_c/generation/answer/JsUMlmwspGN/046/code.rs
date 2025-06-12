// Answer 0

#[test]
fn test_decode_utf8_invalid_byte_count() {
    let input: Vec<u8> = vec![0b11110_000]; // Only one byte, should return None
    assert_eq!(decode_utf8(&input), None);
}

#[test]
fn test_decode_utf8_invalid_continuation_bytes() {
    let input: Vec<u8> = vec![0b11110_000, 0b10_000000]; // Valid start byte, invalid continuation byte
    assert_eq!(decode_utf8(&input), None);
}

#[test]
fn test_decode_utf8_too_short_for_four_byte_sequence() {
    let input: Vec<u8> = vec![0b11110_000, 0b10_000000, 0b10_000000]; // Short for valid four-byte sequence
    assert_eq!(decode_utf8(&input), None);
}

#[test]
fn test_decode_utf8_invalid_shorter_sequence() {
    let input: Vec<u8> = vec![0b11110_000, 0b10_000000, 0b10_000000, 0b10_000000]; // Four bytes but invalid
    assert_eq!(decode_utf8(&input), None);
}

#[test]
fn test_decode_utf8_four_byte_surrogate() {
    let input: Vec<u8> = vec![0b11110_000, 0b10_000000, 0b10_000000, 0b10_000000]; // Surrogate code point: U+D800 is invalid
    assert_eq!(decode_utf8(&input), None);
}

