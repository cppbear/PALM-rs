// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    assert!(!starts_with_digit(""));
}

#[test]
fn test_starts_with_digit_non_digit_first_char() {
    assert!(!starts_with_digit("A123"));
}

#[test]
fn test_starts_with_digit_non_digit_first_char_lowercase() {
    assert!(!starts_with_digit("a123"));
}

#[test]
fn test_starts_with_digit_digit_first_char() {
    assert!(starts_with_digit("1xyz"));
}

#[test]
fn test_starts_with_digit_digit_first_char_zero() {
    assert!(starts_with_digit("0abc"));
}

#[test]
fn test_starts_with_digit_digit_first_char_nine() {
    assert!(starts_with_digit("9xyz"));
}

#[test]
fn test_starts_with_digit_special_characters() {
    assert!(!starts_with_digit("#123"));
}

