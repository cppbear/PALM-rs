// Answer 0

#[test]
fn test_push_wtf8_codepoint_case_1() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_case_2() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1011_1111, 0b1011_1111]);
}

#[test]
fn test_push_wtf8_codepoint_case_3() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xABCDEF, &mut scratch);
    assert_eq!(scratch, vec![0b1111_0000, 0b1000_1010, 0b1011_1100, 0b1010_1111]);
}

