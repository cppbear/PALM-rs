// Answer 0

#[test]
fn test_decode_utf8_valid_two_bytes() {
    let input = [0b11000010, 0b10111000]; // U+00B8
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('¸', 2)));
}

#[test]
fn test_decode_utf8_valid_three_bytes() {
    let input = [0b11100000, 0b10111000, 0b10111111]; // U+07BF
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('߿', 3)));
}

#[test]
fn test_decode_utf8_valid_four_bytes() {
    let input = [0b11110000, 0b10010000, 0b10000000, 0b10000000]; // U+10000
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('𐀀', 4)));
}

#[test]
fn test_decode_utf8_too_short_for_two_bytes() {
    let input = [0b11000010]; // Only one byte
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_second_byte() {
    let input = [0b11000010, 0b10111001]; // Valid first byte, invalid second byte
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_too_short_for_three_bytes() {
    let input = [0b11100000, 0b10111000]; // Only two bytes
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_three_bytes() {
    let input = [0b11100000, 0b10111000, 0b00111111]; // Invalid third byte
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_too_short_for_four_bytes() {
    let input = [0b11110000, 0b10010000, 0b10000000]; // Only three bytes
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_four_bytes() {
    let input = [0b11110000, 0b10010000, 0b10000000, 0b00111111]; // Invalid fourth byte
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

