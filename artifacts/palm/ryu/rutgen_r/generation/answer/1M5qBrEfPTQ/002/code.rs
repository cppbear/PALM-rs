// Answer 0

#[test]
fn test_decimal_length17_high_boundary() {
    let v: u64 = 1000000000000000; // v is 17 digits, right on the edge of being considered.
    let result = decimal_length17(v);
    assert_eq!(result, 16); // We expect the function to return 16 for this value.
}

#[test]
fn test_decimal_length17_mid_boundary() {
    let v: u64 = 999999999999999; // v is less than 17 digits, should return 15.
    let result = decimal_length17(v);
    assert_eq!(result, 15);
}

#[test]
fn test_decimal_length17_lower_boundary() {
    let v: u64 = 1000000000000; // v is exactly 13 digits.
    let result = decimal_length17(v);
    assert_eq!(result, 13);
}

