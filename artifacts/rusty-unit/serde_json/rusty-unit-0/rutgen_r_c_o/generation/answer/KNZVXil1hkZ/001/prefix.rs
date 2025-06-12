// Answer 0

#[test]
fn test_push_wtf8_codepoint_low() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_mid() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(65, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_high() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(127, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_multiple() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(10, &mut scratch);
    push_wtf8_codepoint(20, &mut scratch);
    push_wtf8_codepoint(30, &mut scratch);
}

