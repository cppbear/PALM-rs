// Answer 0

#[test]
fn test_push_wtf8_codepoint_1() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_2() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x20000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_3() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x30000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_4() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x40000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_5() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch);
}

