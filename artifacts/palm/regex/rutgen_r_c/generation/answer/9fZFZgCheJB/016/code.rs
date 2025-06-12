// Answer 0

#[test]
fn test_is_capture_char_with_valid_uppercase_first_character() {
    let result = is_capture_char('A', true);
    assert_eq!(result, true);
}

#[test]
fn test_is_capture_char_with_valid_uppercase_following_character() {
    let result = is_capture_char('B', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_capture_char_with_lowercase_character() {
    let result = is_capture_char('c', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_capture_char_with_digit_first_character() {
    let result = is_capture_char('3', true);
    assert_eq!(result, false);
}

#[test]
fn test_is_capture_char_with_digit_following_character() {
    let result = is_capture_char('2', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_capture_char_with_underscore_first_character() {
    let result = is_capture_char('_', true);
    assert_eq!(result, true);
}

#[test]
fn test_is_capture_char_with_underscore_following_character() {
    let result = is_capture_char('_', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_capture_char_with_invalid_character() {
    let result = is_capture_char('!', false);
    assert_eq!(result, false);
}

#[test]
fn test_is_capture_char_with_character_below_lower_bound() {
    let result = is_capture_char('@', false);
    assert_eq!(result, false);
}

#[test]
fn test_is_capture_char_with_character_above_upper_bound() {
    let result = is_capture_char('[', false);
    assert_eq!(result, false);
}

