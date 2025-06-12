// Answer 0

#[test]
fn test_decode_utf8_two_byte_invalid() {
    let input: &[u8] = &[0b110_00001, 0b101_00000]; // valid first byte, but invalid second byte (does not follow the TAG_CONT)
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_two_byte_valid() {
    let input: &[u8] = &[0b110_00010, 0b10000010]; // valid UTF-8 for '¢'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('¢', 2)));
}

#[test]
fn test_decode_utf8_three_byte_invalid() {
    let input: &[u8] = &[0b1110_0001, 0b10000000, 0b10100000]; // valid first byte, invalid second byte (does not follow the TAG_CONT)
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_three_byte_valid() {
    let input: &[u8] = &[0b1110_0001, 0b10000001, 0b10100001]; // valid UTF-8 for '¢'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('¢', 3)));
}

#[test]
fn test_decode_utf8_four_byte_invalid() {
    let input: &[u8] = &[0b11110_000, 0b10000000, 0b10000000, 0b10100000]; // valid first byte, invalid second byte (does not follow the TAG_CONT)
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_four_byte_valid() {
    let input: &[u8] = &[0b11110_000, 0b10000000, 0b10000001, 0b10100001]; // valid UTF-8 for '𐍈'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('𐍈', 4)));
}

