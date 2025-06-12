// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    let slice = "";
    assert_eq!(starts_with_digit(slice), false);
}

#[test]
fn test_starts_with_digit_non_digit_characters() {
    let slice = "abc";
    assert_eq!(starts_with_digit(slice), false);
}

#[test]
fn test_starts_with_digit_space() {
    let slice = " ";
    assert_eq!(starts_with_digit(slice), false);
}

#[test]
fn test_starts_with_digit_special_characters() {
    let slice = "!@#$%^&*()";
    assert_eq!(starts_with_digit(slice), false);
}

