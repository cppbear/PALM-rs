// Answer 0

#[test]
fn test_decode_utf8_invalid_too_short() {
    let input = [0b11110_000]; // starting byte indicating 4-byte sequence but only 1 byte provided
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_case() {
    let input = [0b11110_000, 0b1000_0000, 0b1000_0000, 0b1000_0000]; // valid 4-byte sequence for U+100000
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('𐀀', 4)));
}

#[test]
fn test_decode_utf8_invalid_not_enough_bytes() {
    let input = [0b11110_001, 0b1000_0000, 0b1000_0000]; // missing one byte for a valid 4-byte sequence
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

