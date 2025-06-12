// Answer 0

#[test]
fn test_push_wtf8_codepoint_below_0x80() {
    let mut scratch: Vec<u8> = Vec::new();
    let codepoint: u32 = 0x7F;  // The maximum value that satisfies n < 0x80

    push_wtf8_codepoint(codepoint, &mut scratch);
    
    assert_eq!(scratch.len(), 1);
    assert_eq!(scratch[0], codepoint as u8);
}

#[test]
fn test_push_wtf8_codepoint_zero() {
    let mut scratch: Vec<u8> = Vec::new();
    let codepoint: u32 = 0x00; // Testing the lowest valid codepoint

    push_wtf8_codepoint(codepoint, &mut scratch);
    
    assert_eq!(scratch.len(), 1);
    assert_eq!(scratch[0], codepoint as u8);
}

#[test]
fn test_push_wtf8_codepoint_negative() {
    let mut scratch: Vec<u8> = Vec::new();
    let codepoint: u32 = 0x7F; // Testing the upper boundary for n < 0x80

    push_wtf8_codepoint(codepoint, &mut scratch);

    assert_eq!(scratch.len(), 1);
    assert_eq!(scratch[0], codepoint as u8);
}

