// Answer 0

#[test]
fn test_push_wtf8_codepoint_ascii() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x41, &mut scratch); // ASCII 'A'
    assert_eq!(scratch, vec![0x41]);
}

#[test]
fn test_push_wtf8_codepoint_two_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xC2A9, &mut scratch); // ©
    assert_eq!(scratch, vec![0xC2, 0xA9]);
}

#[test]
fn test_push_wtf8_codepoint_three_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xE2A9, &mut scratch); // 𐍉
    assert_eq!(scratch, vec![0xE2, 0xA9, 0xB9]);
}

#[test]
fn test_push_wtf8_codepoint_four_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch); // valid maximum Unicode code point
    assert_eq!(scratch, vec![0xF4, 0x8F, 0xBF, 0xBF]);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_invalid() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x11_0000, &mut scratch); // This should panic as it's out of valid range.
}

#[test]
fn test_push_wtf8_codepoint_empty_buffer() {
    let mut scratch = Vec::new();
    assert!(scratch.is_empty());
    push_wtf8_codepoint(0x41, &mut scratch); // ASCII 'A'
    assert_eq!(scratch.len(), 1);
}

#[test]
fn test_push_wtf8_codepoint_multiple_calls() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x41, &mut scratch); // 'A'
    push_wtf8_codepoint(0xC2A9, &mut scratch); // ©
    push_wtf8_codepoint(0x10FFFF, &mut scratch); // valid maximum Unicode code point
    assert_eq!(scratch, vec![0x41, 0xC2, 0xA9, 0xF4, 0x8F, 0xBF, 0xBF]);
}

