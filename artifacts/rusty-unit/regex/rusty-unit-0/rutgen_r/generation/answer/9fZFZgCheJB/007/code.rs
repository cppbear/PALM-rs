// Answer 0

#[test]
fn test_is_capture_char_valid_first_character() {
    assert_eq!(is_capture_char('a', true), true);
    assert_eq!(is_capture_char('Z', true), true);
    assert_eq!(is_capture_char('_', true), true);
    assert_eq!(is_capture_char('1', true), false);
}

#[test]
fn test_is_capture_char_valid_non_first_character() {
    assert_eq!(is_capture_char('a', false), true);
    assert_eq!(is_capture_char('Z', false), true);
    assert_eq!(is_capture_char('_', false), true);
    assert_eq!(is_capture_char('0', false), true);
    assert_eq!(is_capture_char('9', false), true);
}

#[test]
fn test_is_capture_char_invalid_characters() {
    assert_eq!(is_capture_char('!', true), false);
    assert_eq!(is_capture_char('@', false), false);
    assert_eq!(is_capture_char('#', false), false);
    assert_eq!(is_capture_char(' ', false), false);
}

