// Answer 0

#[test]
fn test_starts_with_digit_empty() {
    assert_eq!(starts_with_digit(""), false);
}

#[test]
fn test_starts_with_digit_non_digit() {
    assert_eq!(starts_with_digit("abc"), false);
}

#[test]
fn test_starts_with_digit_starting_with_zero() {
    assert_eq!(starts_with_digit("0abc"), true);
}

#[test]
fn test_starts_with_digit_starting_with_nine() {
    assert_eq!(starts_with_digit("9abc"), true);
}

#[test]
fn test_starts_with_digit_starting_with_space() {
    assert_eq!(starts_with_digit(" abc"), false);
}

#[test]
fn test_starts_with_digit_special_character() {
    assert_eq!(starts_with_digit("@abc"), false);
}

#[test]
fn test_starts_with_digit_numeric_string() {
    assert_eq!(starts_with_digit("12345"), true);
}

