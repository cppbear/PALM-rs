// Answer 0

#[test]
fn test_push_wtf8_codepoint_80_to_7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_81_to_7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x81, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_200_to_7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x200, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_400_to_7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x400, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_800_to_7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_FF00_to_7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFF00, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_7FF_to_7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7FF, &mut scratch);
}

