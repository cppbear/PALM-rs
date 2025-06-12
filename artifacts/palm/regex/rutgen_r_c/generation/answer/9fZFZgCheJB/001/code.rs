// Answer 0

#[test]
fn test_is_capture_char_not_first_digit() {
    assert_eq!(is_capture_char('0', false), true);
}

#[test]
fn test_is_capture_char_first_digit() {
    assert_eq!(is_capture_char('0', true), false);
}

#[test]
fn test_is_capture_char_lowercase_a() {
    assert_eq!(is_capture_char('a', false), true);
    assert_eq!(is_capture_char('a', true), true);
}

#[test]
fn test_is_capture_char_uppercase_a() {
    assert_eq!(is_capture_char('A', false), true);
    assert_eq!(is_capture_char('A', true), true);
}

#[test]
fn test_is_capture_char_underscore() {
    assert_eq!(is_capture_char('_', false), true);
    assert_eq!(is_capture_char('_', true), true);
}

#[test]
fn test_is_capture_char_invalid_char() {
    assert_eq!(is_capture_char('@', false), false);
    assert_eq!(is_capture_char('@', true), false);
}

#[test]
fn test_is_capture_char_boundary_conditions() {
    assert_eq!(is_capture_char('9', false), true);
    assert_eq!(is_capture_char('9', true), false);
    assert_eq!(is_capture_char('Z', false), true);
    assert_eq!(is_capture_char('Z', true), true);
}

