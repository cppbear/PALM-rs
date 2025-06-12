// Answer 0

#[test]
fn test_is_capture_char_first_valid() {
    assert_eq!(is_capture_char('a', true), true);
    assert_eq!(is_capture_char('Z', true), true);
    assert_eq!(is_capture_char('_', true), true);
}

#[test]
fn test_is_capture_char_first_invalid() {
    assert_eq!(is_capture_char('0', true), false);
    assert_eq!(is_capture_char('9', true), false);
}

#[test]
fn test_is_capture_char_not_first_valid() {
    assert_eq!(is_capture_char('1', false), true);
    assert_eq!(is_capture_char('b', false), true);
    assert_eq!(is_capture_char('Z', false), true);
    assert_eq!(is_capture_char('_', false), true);
}

#[test]
fn test_is_capture_char_not_first_invalid() {
    assert_eq!(is_capture_char('!', false), false);
    assert_eq!(is_capture_char('@', false), false);
    assert_eq!(is_capture_char(' ', false), false);
}

