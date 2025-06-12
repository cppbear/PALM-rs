// Answer 0

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_below_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7F, &mut scratch);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_between_0x80_and_0x7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_between_0x800_and_0xFFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_between_0x10000_and_0x10FFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10000, &mut scratch);
    assert_eq!(scratch.len(), 4);
}

#[test]
fn test_push_wtf8_codepoint_at_0x10FFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch);
    assert_eq!(scratch.len(), 4);
}

