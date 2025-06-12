// Answer 0

#[test]
fn test_push_wtf8_codepoint_below_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7F, &mut scratch);
    assert_eq!(scratch, vec![0x7F]);
}

#[test]
fn test_push_wtf8_codepoint_just_above_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_within_0x80_to_0x7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7FF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000, 0b1011_1111]);
}

#[test]
fn test_push_wtf8_codepoint_within_0x800_to_0xFFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1011_1111, 0b1011_1111]);
}

#[test]
fn test_push_wtf8_codepoint_within_0x10000_to_0x10FFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1111_0000, 0b1000_0010, 0b1000_0000, 0b1000_0011]);
}

