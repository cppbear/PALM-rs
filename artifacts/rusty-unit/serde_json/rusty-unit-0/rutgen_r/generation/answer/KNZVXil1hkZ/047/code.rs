// Answer 0

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_out_of_bounds() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x110000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_boundary_case() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch);
    assert_eq!(scratch.len(), 4);
    assert_eq!(scratch[0], 0xF0);
    assert_eq!(scratch[1], 0x9F);
    assert_eq!(scratch[2], 0xBF);
    assert_eq!(scratch[3], 0xBF);
}

#[test]
fn test_push_wtf8_codepoint_maximum() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10000, &mut scratch);
    assert_eq!(scratch.len(), 4);
    assert_eq!(scratch[0], 0xF0);
    assert_eq!(scratch[1], 0x90);
    assert_eq!(scratch[2], 0x80);
    assert_eq!(scratch[3], 0x80);
}

