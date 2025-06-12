// Answer 0

#[test]
fn test_push_wtf8_codepoint_0x800() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_0xD800() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xD800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_1101, 0b1000_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_0xFFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_1111, 0b1011_1111, 0b1011_1111]);
}

#[test]
fn test_push_wtf8_codepoint_0x10000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10000, &mut scratch);
    assert_eq!(scratch, vec![0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000]);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_invalid_low() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x11_0000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_empty() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
    push_wtf8_codepoint(0xFFFF, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000, 0b1110_1111, 0b1011_1111]);
}

