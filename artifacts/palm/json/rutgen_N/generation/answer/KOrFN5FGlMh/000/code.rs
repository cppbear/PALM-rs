// Answer 0

#[test]
fn test_starts_with_digit_empty() {
    assert_eq!(starts_with_digit(""), false);
}

#[test]
fn test_starts_with_digit_non_digit_start() {
    assert_eq!(starts_with_digit("abc"), false);
}

#[test]
fn test_starts_with_digit_digit_start() {
    assert_eq!(starts_with_digit("1abc"), true);
}

#[test]
fn test_starts_with_digit_zero() {
    assert_eq!(starts_with_digit("0abc"), true);
}

#[test]
fn test_starts_with_digit_multiple_digits() {
    assert_eq!(starts_with_digit("123"), true);
}

#[test]
fn test_starts_with_digit_space_start() {
    assert_eq!(starts_with_digit(" abc"), false);
}

#[test]
fn test_starts_with_digit_special_char_start() {
    assert_eq!(starts_with_digit("@abc"), false);
}

