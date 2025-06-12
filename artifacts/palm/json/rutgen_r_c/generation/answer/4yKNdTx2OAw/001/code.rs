// Answer 0

#[test]
fn test_decode_four_hex_digits_with_zero() {
    let a: u8 = 0x00; // '0'
    let b: u8 = 0x00; // '0'
    let c: u8 = 0x00; // '0'
    let d: u8 = 0x00; // '0'

    let result = decode_four_hex_digits(a, b, c, d);
    assert_eq!(result, Some(0u16)); // codepoint == 0
}

#[test]
fn test_decode_four_hex_digits_with_min_positive() {
    let a: u8 = 0x00; // '0'
    let b: u8 = 0x01; // '1'
    let c: u8 = 0x00; // '0'
    let d: u8 = 0x00; // '0'

    let result = decode_four_hex_digits(a, b, c, d);
    assert_eq!(result, Some(256u16)); // codepoint == 256
}

#[test]
fn test_decode_four_hex_digits_with_edge_case() {
    let a: u8 = 0x0F; // 'F'
    let b: u8 = 0x00; // '0'
    let c: u8 = 0x00; // '0'
    let d: u8 = 0x00; // '0'

    let result = decode_four_hex_digits(a, b, c, d);
    assert_eq!(result, Some(3840u16)); // codepoint == 3840 (0x0F00)
}

#[test]
fn test_decode_four_hex_digits_with_maximum() {
    let a: u8 = 0xFF; // should trigger maximum value, but yield valid output
    let b: u8 = 0xFF; // should trigger maximum value, but yield valid output
    let c: u8 = 0xFF; // should trigger maximum value, but yield valid output
    let d: u8 = 0xFF; // should trigger maximum value, but yield valid output

    let result = decode_four_hex_digits(a, b, c, d);
    assert_eq!(result, Some(65535u16)); // codepoint == 65535 (0xFFFF)
}

