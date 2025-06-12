// Answer 0

#[test]
fn test_push_wtf8_codepoint_boundary_0x800() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000]); // Expectation for 0x800
}

#[test]
fn test_push_wtf8_codepoint_at_upper_boundary_0xFFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1011_1111, 0b1001_1111]); // Expectation for 0xFFFF
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_below_lower_boundary() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7F, &mut scratch); // Should panic due to constraint violation
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_above_upper_boundary() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x110000, &mut scratch); // Should panic due to constraint violation
}

#[test]
fn test_push_wtf8_codepoint_within_range_0x801() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x801, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0001, 0b1000_0000]); // Expectation for 0x801
}

