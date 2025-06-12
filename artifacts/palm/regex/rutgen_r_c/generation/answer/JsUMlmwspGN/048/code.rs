// Answer 0

#[test]
fn test_decode_utf8_four_bytes_invalid() {
    let input: &[u8] = &[0b11110_0000, 0b10000000, 0b10000001, 0b11000000]; // Invalid because b2 does not have the expected continuation byte format
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_four_bytes_valid() {
    let input: &[u8] = &[0b11110_0000, 0b10000000, 0b10000000, 0b10000001]; // Valid UTF-8, represents U+100000
    let result = decode_utf8(input);
    assert_eq!(result, Some((char::from_u32(0x100000).unwrap(), 4)));
}

#[test]
fn test_decode_utf8_four_bytes_incomplete() {
    let input: &[u8] = &[0b11110_0000, 0b10000000, 0b10000000]; // Too few bytes
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_three_bytes_invalid_continuation() {
    let input: &[u8] = &[0b1110_0000, 0b10000000, 0b10000010]; // Valid first byte but invalid continuation
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_two_bytes_valid() {
    let input: &[u8] = &[0b110_00010, 0b10000011]; // Valid UTF-8, represents U+0233
    let result = decode_utf8(input);
    assert_eq!(result, Some((char::from_u32(0x0233).unwrap(), 2)));
}

#[test]
fn test_decode_utf8_two_bytes_incomplete() {
    let input: &[u8] = &[0b110_00010]; // Too few bytes
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_single_byte() {
    let input: &[u8] = &[0b01111000]; // Single byte valid (U+78)
    let result = decode_utf8(input);
    assert_eq!(result, Some((char::from_u32(0x78).unwrap(), 1)));
}

#[test]
fn test_decode_utf8_empty_input() {
    let input: &[u8] = &[]; // Empty input
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

