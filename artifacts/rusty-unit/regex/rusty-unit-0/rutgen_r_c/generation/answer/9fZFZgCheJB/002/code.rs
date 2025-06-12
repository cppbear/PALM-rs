// Answer 0

#[test]
fn test_is_capture_char() {
    // Test when c is '_' and first is true
    assert_eq!(is_capture_char('_', true), true);
    
    // Test when c is 'a' and first is true
    assert_eq!(is_capture_char('a', true), true);
    
    // Test when c is 'z' and first is true
    assert_eq!(is_capture_char('z', true), true);
    
    // Test when c is 'A' and first is true
    assert_eq!(is_capture_char('A', true), true);
    
    // Test when c is 'Z' and first is true
    assert_eq!(is_capture_char('Z', true), true);
    
    // Boundary test when c is '0' and first is false
    assert_eq!(is_capture_char('0', false), true);
    
    // Boundary test when c is '9' and first is false
    assert_eq!(is_capture_char('9', false), true);
    
    // Test when c is '0' and first is true, expect false
    assert_eq!(is_capture_char('0', true), false);
    
    // Test when c is ' ' (space) and first is true, expect false
    assert_eq!(is_capture_char(' ', true), false);
    
    // Test when c is '-' (dash) and first is true, expect false
    assert_eq!(is_capture_char('-', true), false);
}

