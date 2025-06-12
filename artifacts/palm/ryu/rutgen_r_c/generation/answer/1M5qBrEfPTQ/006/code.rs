// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let value = 100_000_000_000; // 12 digits, meets the condition v < 100000000000000000
    let result = decimal_length17(value);
    assert_eq!(result, 12);
}

#[test]
fn test_decimal_length17_case_2() {
    let value = 99_999_999_999; // 11 digits, below the threshold of 12
    let result = decimal_length17(value);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_case_3() {
    let value = 9_999_999_999; // 10 digits
    let result = decimal_length17(value);
    assert_eq!(result, 10);
}

#[test]
fn test_decimal_length17_case_4() {
    let value = 1_000_000_000; // 10 digits
    let result = decimal_length17(value);
    assert_eq!(result, 10);
}

#[test]
fn test_decimal_length17_minimum() {
    let value = 1; // 1 digit
    let result = decimal_length17(value);
    assert_eq!(result, 1);
}

