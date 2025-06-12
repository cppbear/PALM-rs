// Answer 0

#[test]
fn test_decimal_length17_lower_bound() {
    let value: u64 = 1000000000000;
    let result = decimal_length17(value);
    assert_eq!(result, 13);
}

#[test]
fn test_decimal_length17_upper_bound() {
    let value: u64 = 99999999999999999; // Value is less than 100000000000000000
    let result = decimal_length17(value);
    assert_eq!(result, 17);
}

#[test]
fn test_decimal_length17_exactly_17_digits() {
    let value: u64 = 10000000000000000; // Should fall into 16-digit category
    let result = decimal_length17(value);
    assert_eq!(result, 16);
}

#[test]
fn test_decimal_length17_high_range() {
    let value: u64 = 9999999999999999; // Should return 17
    let result = decimal_length17(value);
    assert_eq!(result, 17);
}

#[test]
fn test_decimal_length17_mid_range() {
    let value: u64 = 10000000000000; // Should return 14
    let result = decimal_length17(value);
    assert_eq!(result, 14);
}

#[test]
fn test_decimal_length17_low_range() {
    let value: u64 = 999; // Should return 4
    let result = decimal_length17(value);
    assert_eq!(result, 4);
}

