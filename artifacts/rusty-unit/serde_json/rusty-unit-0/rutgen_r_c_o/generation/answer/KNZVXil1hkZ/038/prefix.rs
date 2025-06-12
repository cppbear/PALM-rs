// Answer 0

#[test]
fn test_push_wtf8_codepoint_case_1() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_case_2() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x801, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_case_3() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_case_4() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x1000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_case_5() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
}

