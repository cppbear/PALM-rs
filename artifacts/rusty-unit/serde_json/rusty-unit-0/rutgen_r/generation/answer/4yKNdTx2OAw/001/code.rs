// Answer 0

#[test]
fn test_decode_four_hex_digits_zero() {
    const HEX0: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const HEX1: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let result = decode_four_hex_digits(0, 0, 0, 0);
    assert_eq!(result, Some(0));
}

#[test]
fn test_decode_four_hex_digits_boundary_condition() {
    const HEX0: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const HEX1: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let result = decode_four_hex_digits(15, 15, 15, 15);
    assert!(result.is_none());
}

#[test]
fn test_decode_four_hex_digits_maximal_codepoint() {
    const HEX0: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const HEX1: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let result = decode_four_hex_digits(1, 0, 0, 0);
    assert_eq!(result, Some(256)); // 256 = (1 << 8) + 0 + 0
}

#[test]
fn test_decode_four_hex_digits_minimal_codepoint() {
    const HEX0: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const HEX1: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let result = decode_four_hex_digits(0, 0, 0, 1);
    assert_eq!(result, Some(1)); // 1 = (0 << 8) + 0 + 1
}

