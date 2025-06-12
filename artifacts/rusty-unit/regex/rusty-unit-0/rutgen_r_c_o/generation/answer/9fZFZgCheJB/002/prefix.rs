// Answer 0

#[test]
fn test_is_capture_char_with_underscore_as_first() {
    let result = is_capture_char('_', true);
}

#[test]
fn test_is_capture_char_with_first_character_a() {
    let result = is_capture_char('a', true);
}

#[test]
fn test_is_capture_char_with_first_character_z() {
    let result = is_capture_char('z', true);
}

#[test]
fn test_is_capture_char_with_first_character_A() {
    let result = is_capture_char('A', true);
}

#[test]
fn test_is_capture_char_with_first_character_Z() {
    let result = is_capture_char('Z', true);
}

#[test]
fn test_is_capture_char_with_digit_not_first() {
    let result = is_capture_char('0', false);
}

#[test]
fn test_is_capture_char_with_non_capture_char_not_first() {
    let result = is_capture_char('-', false);
}

