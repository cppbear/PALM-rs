// Answer 0

#[test]
fn test_is_capture_char_first_char_valid() {
    assert!(is_capture_char('_', true));
    assert!(is_capture_char('a', true));
    assert!(is_capture_char('Z', true));
    assert!(!is_capture_char('1', true));
    assert!(!is_capture_char('!', true));
}

#[test]
fn test_is_capture_char_subsequent_char_valid() {
    assert!(is_capture_char('0', false));
    assert!(is_capture_char('9', false));
    assert!(is_capture_char('b', false));
    assert!(is_capture_char('A', false));
    assert!(is_capture_char('_', false));
    assert!(!is_capture_char('!', false));
    assert!(!is_capture_char(' ', false));
}

#[test]
fn test_is_capture_char_boundary_conditions() {
    assert!(!is_capture_char('1', true));
    assert!(is_capture_char('a', true));
    assert!(is_capture_char('_', true));
    assert!(!is_capture_char('!', false));
    assert!(!is_capture_char('0', true));
}

