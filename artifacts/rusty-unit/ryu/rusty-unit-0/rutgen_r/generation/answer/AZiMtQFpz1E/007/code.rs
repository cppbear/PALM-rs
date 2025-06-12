// Answer 0

#[test]
fn test_decimal_length9() {
    let v: u32 = 100; // Constraint: v >= 100 is true, with bound v == 100
    let expected_length = 3;
    let result = decimal_length9(v);
    assert_eq!(result, expected_length);
}

#[test]
fn test_decimal_length9_min_bound() {
    let v: u32 = 1; // Constraint: v < 100 (less than 100)
    let expected_length = 1;
    let result = decimal_length9(v);
    assert_eq!(result, expected_length);
}

#[test]
fn test_decimal_length9_upper_bound() {
    let v: u32 = 999999999; // Maximum value just below 10 digits
    let expected_length = 9;
    let result = decimal_length9(v);
    assert_eq!(result, expected_length);
}

