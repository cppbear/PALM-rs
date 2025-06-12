// Answer 0

#[test]
fn test_is_capture_char() {
    assert_eq!(is_capture_char('_', true), true);  // c == '_', first is true
    assert_eq!(is_capture_char('0', false), true); // c >= '0' is true, first is false
    assert_eq!(is_capture_char('1', false), true); // c >= '0' is true, first is false
    assert_eq!(is_capture_char('2', false), true); // c >= '0' is true, first is false
    assert_eq!(is_capture_char('a', false), true); // c >= 'a' is true, first is false
    assert_eq!(is_capture_char('z', false), true); // c <= 'z' is false; c is within range
    assert_eq!(is_capture_char('A', false), false); // c >= 'A' is false, first is false
}

