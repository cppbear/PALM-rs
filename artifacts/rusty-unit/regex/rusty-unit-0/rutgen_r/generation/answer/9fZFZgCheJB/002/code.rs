// Answer 0

#[test]
fn test_is_capture_char() {
    // Test for valid underscore when `first` is true
    assert!(is_capture_char('_', true));
    
    // Test for valid first character 'a' when `first` is true
    assert!(is_capture_char('a', true));
    
    // Test for valid first character 'A' when `first` is true
    assert!(is_capture_char('A', true));

    // Test for valid last character 'z' when `first` is true
    assert!(is_capture_char('z', true));

    // Test for valid last character 'Z' when `first` is true
    assert!(is_capture_char('Z', true));

    // Test for invalid first character '0' when `first` is true
    assert!(!is_capture_char('0', true));

    // Test for valid character '1' when `first` is false
    assert!(is_capture_char('1', false));
    
    // Test for valid character '9' when `first` is false
    assert!(is_capture_char('9', false));

    // Test for invalid character '0' when `first` is false
    assert!(!is_capture_char('0', false));
    
    // Test for valid character 'b' when `first` is true
    assert!(is_capture_char('b', true));

    // Test for invalid character '-' when `first` is false
    assert!(!is_capture_char('-', false));
}

