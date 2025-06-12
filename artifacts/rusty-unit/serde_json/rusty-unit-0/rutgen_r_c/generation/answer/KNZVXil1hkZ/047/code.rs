// Answer 0

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_invalid_surrogate() {
    let mut scratch = Vec::new();
    // Test boundary condition where n is a value that is greater than the maximum valid UTF-8 code point.
    push_wtf8_codepoint(0x110000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_valid_case() {
    let mut scratch = Vec::new();
    // Test a valid UTF-8 codepoint within the range 0x10000 to 0x10FFFF.
    push_wtf8_codepoint(0x10000, &mut scratch);
    // Check that the length is now 4 after inserting the codepoint.
    assert_eq!(scratch.len(), 4);
} 

#[test]
fn test_push_wtf8_codepoint_edge_case() {
    let mut scratch = Vec::new();
    // Testing a codepoint that is exactly 0x11_0000, which should trigger a panic.
    push_wtf8_codepoint(0x11_0000, &mut scratch);
}

