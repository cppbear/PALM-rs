// Answer 0

#[test]
fn test_push_wtf8_codepoint_ascii() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x41, &mut scratch); // 'A'
    assert_eq!(scratch, vec![0x41]);
}

#[test]
fn test_push_wtf8_codepoint_two_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xC2A2, &mut scratch); // '¢' (U+00A2)
    assert_eq!(scratch, vec![0xC2, 0xA2]);
}

#[test]
fn test_push_wtf8_codepoint_three_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xE29C82, &mut scratch); // '🌺' (U+1F33A)
    assert_eq!(scratch, vec![0xF0, 0x9F, 0x90, 0xBA]);
}

#[test]
fn test_push_wtf8_codepoint_four_bytes() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch); // Last valid code point (U+10FFFF)
    assert_eq!(scratch, vec![0xF4, 0x8F, 0xBF, 0xBF]);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_out_of_bounds() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x110000, &mut scratch); // Out of bounds
}

