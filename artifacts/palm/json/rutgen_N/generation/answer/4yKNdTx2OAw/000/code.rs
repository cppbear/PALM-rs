// Answer 0

#[test]
fn test_decode_four_hex_digits_valid() {
    assert_eq!(decode_four_hex_digits(0, 0, 1, 1), Some(273)); // 0x0101
    assert_eq!(decode_four_hex_digits(0, 0, 15, 15), Some(4095)); // 0x0FFF
    assert_eq!(decode_four_hex_digits(1, 0, 0, 0), Some(256)); // 0x0100
}

#[test]
fn test_decode_four_hex_digits_invalid() {
    assert_eq!(decode_four_hex_digits(16, 0, 0, 0), None); // Out of bounds
    assert_eq!(decode_four_hex_digits(0, 16, 0, 0), None); // Out of bounds
    assert_eq!(decode_four_hex_digits(0, 0, 16, 0), None); // Out of bounds
    assert_eq!(decode_four_hex_digits(0, 0, 0, 16), None); // Out of bounds
    assert_eq!(decode_four_hex_digits(15, 15, 15, 15), None); // Codepoint exceeds u16
}

#[test]
fn test_decode_four_hex_digits_boundary() {
    assert_eq!(decode_four_hex_digits(15, 15, 15, 14), Some(65534)); // 0xFFFF - 1
    assert_eq!(decode_four_hex_digits(15, 15, 0, 0), Some(61440)); // 0xF000
}

