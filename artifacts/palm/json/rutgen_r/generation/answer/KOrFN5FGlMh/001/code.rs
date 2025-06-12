// Answer 0

#[test]
fn test_starts_with_digit_true() {
    let input = "0abc";
    assert_eq!(starts_with_digit(input), true);
}

#[test]
fn test_starts_with_digit_false() {
    let input = "abc";
    assert_eq!(starts_with_digit(input), false);
}

#[test]
fn test_starts_with_digit_empty() {
    let input = "";
    assert_eq!(starts_with_digit(input), false);
}

#[test]
fn test_starts_with_digit_non_digit() {
    let input = "a2bc";
    assert_eq!(starts_with_digit(input), false);
}

#[test]
fn test_starts_with_digit_boundary() {
    let input_zero = "0";
    let input_nine = "9";
    assert_eq!(starts_with_digit(input_zero), true);
    assert_eq!(starts_with_digit(input_nine), true);
}

