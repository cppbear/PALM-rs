// Answer 0

#[test]
fn test_push_wtf8_codepoint_boundary() {
    let mut scratch = Vec::new();

    // Test the lowest value in the valid range 0x1_0000
    push_wtf8_codepoint(0x1_0000, &mut scratch);
    assert_eq!(scratch, vec![0xF0, 0x90, 0x80, 0x80]);

    // Clear scratch for the next test
    scratch.clear();

    // Test a value just below the upper bound 0x10_FFFF
    push_wtf8_codepoint(0x10_FFFF, &mut scratch);
    assert_eq!(scratch, vec![0xF4, 0x8F, 0xBF, 0xBF]);

    // Clear scratch for the next test
    scratch.clear();

    // Test the upper bound value 0x10_FFFF
    push_wtf8_codepoint(0x10_FFFF, &mut scratch);
    assert_eq!(scratch, vec![0xF4, 0x8F, 0xBF, 0xBF]);
}

