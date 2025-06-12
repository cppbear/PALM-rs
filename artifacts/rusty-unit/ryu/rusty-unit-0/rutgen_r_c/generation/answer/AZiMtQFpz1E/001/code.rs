// Answer 0

#[test]
fn test_decimal_length9_case_9_digits() {
    let v: u32 = 100_000_000;
    let result = decimal_length9(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length9_case_8_digits() {
    let v: u32 = 99_999_999;
    let result = decimal_length9(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length9_case_7_digits() {
    let v: u32 = 9_999_999;
    let result = decimal_length9(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length9_case_6_digits() {
    let v: u32 = 999_999;
    let result = decimal_length9(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length9_case_5_digits() {
    let v: u32 = 99_999;
    let result = decimal_length9(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length9_case_4_digits() {
    let v: u32 = 9_999;
    let result = decimal_length9(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length9_case_3_digits() {
    let v: u32 = 999;
    let result = decimal_length9(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length9_case_2_digits() {
    let v: u32 = 99;
    let result = decimal_length9(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_case_1_digit() {
    let v: u32 = 9;
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

