// Answer 0

#[test]
fn test_push_wtf8_codepoint_below_128() {
    let mut scratch = Vec::new();
    let codepoint: u32 = 0x7F; // Test with the maximum value below 128

    push_wtf8_codepoint(codepoint, &mut scratch);
    
    assert_eq!(scratch.len(), 1); // Expecting one byte added
    assert_eq!(scratch[0], codepoint as u8); // The byte should match the codepoint
}

#[test]
fn test_push_wtf8_codepoint_exactly_128() {
    let mut scratch = Vec::new();
    let codepoint: u32 = 0x80; // Test with the exact boundary of 128
    
    push_wtf8_codepoint(codepoint, &mut scratch);
    
    assert_eq!(scratch.len(), 2); // Expecting two bytes added
    assert_eq!(scratch[0], 0b1100_0000); // The first byte for 0x80 should be 0b1100_0000
    assert_eq!(scratch[1], 0b1000_0000); // The second byte for 0x80 should be 0b1000_0000
}

#[test]
fn test_push_wtf8_codepoint_in_range() {
    let mut scratch = Vec::new();
    let codepoint: u32 = 0x7B; // Test with a value within the valid 0x80..=0x7FF range
    
    push_wtf8_codepoint(codepoint, &mut scratch);
    
    assert_eq!(scratch.len(), 2); // Expecting two bytes added
    assert_eq!(scratch[0], 0b1100_0000 | ((codepoint >> 6) & 0b0001_1111) as u8); // Correct first byte
    assert_eq!(scratch[1], 0b1000_0000 | (codepoint & 0b0011_1111) as u8); // Correct second byte
}

#[test]
fn test_push_wtf8_codepoint_near_upper_bound() {
    let mut scratch = Vec::new();
    let codepoint: u32 = 0x7FF; // Test with the maximum value for 0x80..=0x7FF range
    
    push_wtf8_codepoint(codepoint, &mut scratch);
    
    assert_eq!(scratch.len(), 2); // Expecting two bytes added
    assert_eq!(scratch[0], 0b1100_0000 | ((codepoint >> 6) & 0b0001_1111) as u8); // Correct first byte
    assert_eq!(scratch[1], 0b1000_0000 | (codepoint & 0b0011_1111) as u8); // Correct second byte
}

