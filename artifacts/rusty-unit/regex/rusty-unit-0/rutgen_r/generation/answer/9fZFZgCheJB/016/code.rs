// Answer 0

#[test]
fn test_is_capture_char_valid_first_character() {
    assert_eq!(is_capture_char('A', true), true);
    assert_eq!(is_capture_char('Z', true), true);
}

#[test]
fn test_is_capture_char_invalid_first_character() {
    assert_eq!(is_capture_char('0', true), false);
    assert_eq!(is_capture_char('1', true), false);
}

#[test]
fn test_is_capture_char_valid_non_first_character() {
    assert_eq!(is_capture_char('A', false), true);
    assert_eq!(is_capture_char('Z', false), true);
    assert_eq!(is_capture_char('_', false), true);
}

#[test]
fn test_is_capture_char_invalid_non_first_character() {
    assert_eq!(is_capture_char('0', false), true);
    assert_eq!(is_capture_char('1', false), true);
    assert_eq!(is_capture_char('a', false), false);
    assert_eq!(is_capture_char('z', false), false);
}

#[test]
fn test_is_capture_char_out_of_bounds() {
    assert_eq!(is_capture_char('!', false), false);
    assert_eq!(is_capture_char('@', false), false);
    assert_eq!(is_capture_char('[', false), false);
}

