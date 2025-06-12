// Answer 0

#[test]
fn test_is_capture_char_first_valid() {
    assert!(is_capture_char('a', true));
    assert!(is_capture_char('A', true));
    assert!(is_capture_char('_', true));
}

#[test]
fn test_is_capture_char_first_invalid() {
    assert!(!is_capture_char('1', true));
    assert!(!is_capture_char('!', true));
}

#[test]
fn test_is_capture_char_not_first_valid() {
    assert!(is_capture_char('1', false));
    assert!(is_capture_char('a', false));
    assert!(is_capture_char('A', false));
    assert!(is_capture_char('_', false));
}

#[test]
fn test_is_capture_char_not_first_invalid() {
    assert!(!is_capture_char('!', false));
    assert!(!is_capture_char(' ', false));
}

#[test]
fn test_is_capture_char_edge_cases() {
    assert!(!is_capture_char('0', true));
    assert!(is_capture_char('0', false));
    assert!(!is_capture_char('9', true));
    assert!(is_capture_char('9', false));
}

