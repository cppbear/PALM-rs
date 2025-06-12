// Answer 0

#[test]
fn test_push_wtf8_codepoint_boundary_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_boundary_0x7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7FF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_boundary_0x800() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_boundary_0xFFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_boundary_0x1_0000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x1_0000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_boundary_0x10_FFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10_FFFF, &mut scratch);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_boundary_0x11_0000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x11_0000, &mut scratch);
}

