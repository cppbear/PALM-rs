// Answer 0

#[test]
fn test_push_wtf8_codepoint_boundary_case_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_middle_case() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x200, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0010, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_upper_bound() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7FF, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0111, 0b1011_1111]);
}

#[test]
fn test_push_wtf8_codepoint_three_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_four_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10000, &mut scratch);
    assert_eq!(scratch, vec![0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000]);
}

