// Answer 0

#[test]
fn test_push_wtf8_codepoint_above_range() {
    let n = 0x110000;
    let mut scratch = Vec::new();
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_high_value() {
    let n = 0x200000;
    let mut scratch = Vec::new();
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_mid_value() {
    let n = 0x150000;
    let mut scratch = Vec::new();
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_near_upper_boundary() {
    let n = 0x1FFFFF;
    let mut scratch = Vec::new();
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_exceeding_allowable_range() {
    let n = 0x110001; // Adjust this value if required to test panic on boundary
    let mut scratch = Vec::new();
    push_wtf8_codepoint(n, &mut scratch);
}

