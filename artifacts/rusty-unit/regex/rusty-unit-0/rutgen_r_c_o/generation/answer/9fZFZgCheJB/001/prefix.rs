// Answer 0

#[test]
fn test_is_capture_char_digit_when_not_first() {
    let result1 = is_capture_char('0', false);
    let result2 = is_capture_char('5', false);
    let result3 = is_capture_char('9', false);
}

#[test]
fn test_is_capture_char_lowercase_when_first() {
    let result1 = is_capture_char('a', true);
    let result2 = is_capture_char('m', true);
    let result3 = is_capture_char('z', true);
}

#[test]
fn test_is_capture_char_uppercase_when_first() {
    let result1 = is_capture_char('A', true);
    let result2 = is_capture_char('M', true);
    let result3 = is_capture_char('Z', true);
}

#[test]
fn test_is_capture_char_underscore() {
    let result = is_capture_char('_', true);
}

#[test]
fn test_is_capture_char_invalid() {
    let result1 = is_capture_char('!', true);
    let result2 = is_capture_char('%', false);
    let result3 = is_capture_char('*', true);
}

