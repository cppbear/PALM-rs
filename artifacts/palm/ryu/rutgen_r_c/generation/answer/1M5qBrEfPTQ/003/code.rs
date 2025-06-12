// Answer 0

#[test]
fn test_decimal_length17_bound_case() {
    let value: u64 = 100_000_000_000_000;
    let result = decimal_length17(value);
    assert_eq!(result, 15);
}

#[test]
#[should_panic]
fn test_decimal_length17_exceed_upper_bound() {
    let value: u64 = 100_000_000_000_000_000; // This should panic since it's not allowed
    decimal_length17(value);
}

#[test]
fn test_decimal_length17_mid_boundary_case() {
    let value: u64 = 99_999_999_999_999; // This should return 14
    let result = decimal_length17(value);
    assert_eq!(result, 14);
}

#[test]
fn test_decimal_length17_lower_boundary_case() {
    let value: u64 = 1; // Minimum valid input
    let result = decimal_length17(value);
    assert_eq!(result, 1);
}

