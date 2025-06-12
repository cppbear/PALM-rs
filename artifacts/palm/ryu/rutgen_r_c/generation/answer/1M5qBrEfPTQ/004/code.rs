// Answer 0

#[test]
fn test_decimal_length17_lower_bound() {
    let value = 10000000000000;  // v >= 10000000000000 is true
    let result = decimal_length17(value);
    assert_eq!(result, 14);
}

#[test]
fn test_decimal_length17_boundary() {
    let value = 9999999999999;  // Adjusted to remain within constraints
    let result = decimal_length17(value);
    assert_eq!(result, 14);
}

#[test]
fn test_decimal_length17_zero() {
    let value = 0;  // Lower bound case
    let result = decimal_length17(value);
    assert_eq!(result, 1);
}

#[test]
fn test_decimal_length17_large_number() {
    let value = 99999999999999;  // Upper bound case for 14 digits
    let result = decimal_length17(value);
    assert_eq!(result, 14);
}

