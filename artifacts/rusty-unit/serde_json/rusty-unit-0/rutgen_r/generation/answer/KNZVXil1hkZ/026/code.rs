// Answer 0

#[test]
fn test_push_wtf8_codepoint_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_0x7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7FF, &mut scratch);
    assert_eq!(scratch, vec![0b1101_1111, 0b1011_1111]);
}

#[test]
fn test_push_wtf8_codepoint_0x800() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_0xFFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_1111, 0b1011_1111, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_0x10FFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000]);
}

