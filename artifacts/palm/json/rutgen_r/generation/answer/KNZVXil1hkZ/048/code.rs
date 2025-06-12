// Answer 0

#[test]
fn test_push_wtf8_codepoint_boundary() {
    let mut scratch = Vec::new();

    // Test input where n == 0x80, which should trigger the case for n > 0x7F
    push_wtf8_codepoint(0x80, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0000, 0b1000_0000]);

    // Clear the scratch for the next test
    scratch.clear();

    // Test input where n == 0x7FF, which should also validate proper encoding
    push_wtf8_codepoint(0x7FF, &mut scratch);
    assert_eq!(scratch, vec![0b1101_1111, 0b1011_1111]);

    // Clear the scratch for the next test
    scratch.clear();

    // Test input where n == 0x800, which is the beginning of the next range
    push_wtf8_codepoint(0x800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000, 0b0000_0000]);

    // Clear the scratch for the next test
    scratch.clear();

    // Test input where n == 0xFFFF, which should validate the 3-byte encoding
    push_wtf8_codepoint(0xFFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_1111, 0b1011_1111, 0b1000_0000]);

    // Clear the scratch for the next test
    scratch.clear();

    // Test input where n == 0x10FFFF, which should validate the 4-byte encoding
    push_wtf8_codepoint(0x10FFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000]);
}

