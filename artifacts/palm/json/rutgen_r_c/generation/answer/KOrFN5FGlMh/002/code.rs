// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    assert_eq!(starts_with_digit(""), false);
}

#[test]
fn test_starts_with_digit_non_digit_char() {
    assert_eq!(starts_with_digit("a"), false);
}

#[test]
fn test_starts_with_digit_leading_space() {
    assert_eq!(starts_with_digit(" 1"), false);
}

#[test]
fn test_starts_with_digit_underscore() {
    assert_eq!(starts_with_digit("_1"), false);
}

#[test]
fn test_starts_with_digit_less_than_zero() {
    assert_eq!(starts_with_digit("-1"), false);
}

