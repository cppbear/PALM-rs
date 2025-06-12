// Answer 0

#[test]
fn test_push_wtf8_codepoint_boundary_conditions() {
    let mut scratch = Vec::new();

    // Test the upper boundary of the valid range for code points
    let code_point = 0x10FFFF;
    push_wtf8_codepoint(code_point, &mut scratch);
    assert_eq!(scratch.len(), 4);
    assert_eq!(scratch[0], 0b11110000 | ((code_point >> 18) & 0b0000_0111) as u8);
    assert_eq!(scratch[1], 0b10000000 | ((code_point >> 12) & 0b0011_1111) as u8);
    assert_eq!(scratch[2], 0b10000000 | ((code_point >> 6) & 0b0011_1111) as u8);
    assert_eq!(scratch[3], 0b10000000 | (code_point & 0b0011_1111) as u8);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_unreachable_below_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7F, &mut scratch);  // Should not panic
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_unreachable_above_0x10FFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x110000, &mut scratch);  // Should not panic
}

