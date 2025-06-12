// Answer 0

#[test]
fn test_starts_with_digit_with_zero() {
    let input = "0abc";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

#[test]
fn test_starts_with_digit_with_one() {
    let input = "1abc";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

#[test]
fn test_starts_with_digit_with_nine() {
    let input = "9abc";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

#[test]
fn test_starts_with_digit_with_non_digit() {
    let input = "abc0";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_empty_string() {
    let input = "";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

