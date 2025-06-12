// Answer 0

#[test]
fn test_decode_utf8_invalid_two_bytes() {
    let input = [0b11000010, 0b00111111]; // Invalid second byte for UTF-8
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_two_bytes() {
    let input = [0b11000010, 0b10111111]; // Valid two-byte UTF-8
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('Â', 2)));
}

#[test]
fn test_decode_utf8_invalid_three_bytes() {
    let input = [0b11100010, 0b10111111, 0b00111111]; // Valid first and second byte but invalid continuation
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_three_bytes() {
    let input = [0b11100010, 0b10000010, 0b10111111]; // Valid three-byte UTF-8
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('¢', 3)));
}

#[test]
fn test_decode_utf8_invalid_four_bytes() {
    let input = [0b11110000, 0b10000000, 0b10000000, 0b00111111]; // Valid first three bytes but invalid continuation
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_four_bytes() {
    let input = [0b11110000, 0b10010000, 0b10000001, 0b10000001]; // Valid four-byte UTF-8
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('𐌀', 4)));
}

#[test]
fn test_decode_utf8_empty() {
    let input: &[u8] = &[];
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

