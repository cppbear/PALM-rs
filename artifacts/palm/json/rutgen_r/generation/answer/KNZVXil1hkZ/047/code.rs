// Answer 0

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_large_value() {
    let mut scratch = Vec::new();
    // n is greater than 0x10FFFF, triggering a panic.
    push_wtf8_codepoint(0x11_0000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_boundary_value() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_max_value() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10_FFFF, &mut scratch);
    let expected = vec![0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000];
    assert_eq!(scratch, expected);
}

