// Answer 0

#[test]
fn test_starts_with_digit_true() {
    let input = "5abc";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

#[test]
fn test_starts_with_digit_false_lower() {
    let input = "abc";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_false_upper() {
    let input = "Zebra";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_boundary_zero() {
    let input = "0test";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

#[test]
fn test_starts_with_digit_boundary_nine() {
    let input = "9xyz";
    let result = starts_with_digit(input);
    assert_eq!(result, true);
}

#[test]
fn test_starts_with_digit_empty() {
    let input = "";
    let result = starts_with_digit(input);
    assert_eq!(result, false);
}

