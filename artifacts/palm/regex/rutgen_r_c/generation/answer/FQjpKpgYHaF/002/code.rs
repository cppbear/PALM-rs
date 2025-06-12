// Answer 0

#[test]
fn test_char_debug_empty() {
    // Testing the case where char::from_u32(self.0) matches None
    let empty_char = Char(u32::MAX); // u32::MAX is outside the valid range for char
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", empty_char);
    
    assert!(result.is_ok());
    assert_eq!(output, "Empty");
}

#[test]
fn test_char_debug_invalid_unicode() {
    // Testing additional values that encompass invalid unicode but are less than u32::MAX
    let invalid_char = Char(0x110000); // Value 0x110000 is also outside valid char range
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", invalid_char);
    
    assert!(result.is_ok());
    assert_eq!(output, "Empty");
}

#[test]
fn test_char_debug_boundary_value() {
    // Testing the edge case just above the valid char range
    let boundary_char = Char(0x10FFFF + 1); // Just above the maximum valid char value
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", boundary_char);
    
    assert!(result.is_ok());
    assert_eq!(output, "Empty");
}

