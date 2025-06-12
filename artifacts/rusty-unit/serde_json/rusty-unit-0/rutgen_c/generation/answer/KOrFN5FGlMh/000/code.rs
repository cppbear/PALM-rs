// Answer 0

#[test]
fn test_starts_with_digit_with_digit() {
    let input = "123abc";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

#[test]
fn test_starts_with_digit_with_non_digit() {
    let input = "abc123";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_with_empty_string() {
    let input = "";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_with_special_characters() {
    let input = "!@#";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_with_space() {
    let input = " 123abc";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_with_single_digit() {
    let input = "7";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

