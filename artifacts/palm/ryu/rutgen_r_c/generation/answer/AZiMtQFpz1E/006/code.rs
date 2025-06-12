// Answer 0

#[test]
fn test_decimal_length9_boundary_condition() {
    let value: u32 = 1000; // satisfies all constraints
    let result = decimal_length9(value);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length9_minimum_value() {
    let value: u32 = 1; // edge case, should return 1
    let result = decimal_length9(value);
    assert_eq!(result, 1);
}

#[test]
fn test_decimal_length9_mid_range() {
    let value: u32 = 500; // mid range value, should return 3
    let result = decimal_length9(value);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length9_high_value() {
    let value: u32 = 99999999; // maximum valid value below one billion, should return 8
    let result = decimal_length9(value);
    assert_eq!(result, 8);
}

