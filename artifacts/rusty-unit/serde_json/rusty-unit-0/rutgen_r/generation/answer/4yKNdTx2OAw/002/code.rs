// Answer 0

#[test]
fn test_decode_four_hex_digits_negative_codepoint() {
    const HEX1: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
    const HEX0: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];

    // a, b, c, d will be selected such that the resulting codepoint < 0
    let result = decode_four_hex_digits(0, 0, 0, 0);
    
    assert_eq!(result, None);
}

#[test]
fn test_decode_four_hex_digits_edge_case() {
    const HEX1: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
    const HEX0: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];

    // Using values that cause the codepoint to just touch the negative boundary.
    let result = decode_four_hex_digits(0xF, 0xF, 0xF, 0xF); // Should result in a negative codepoint
    
    assert_eq!(result, None);
}

