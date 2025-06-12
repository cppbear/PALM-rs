// Answer 0

#[test]
fn test_decimal_length17_minimum_case() {
    let v: u64 = 10000000; // Satisfies v >= 10000000
    assert_eq!(decimal_length17(v), 8);
}

#[test]
fn test_decimal_length17_boundary_case_just_below() {
    let v: u64 = 9999999; // Satisfies v < 10000000
    assert_eq!(decimal_length17(v), 7);
}

#[test]
fn test_decimal_length17_zero_case() {
    let v: u64 = 0; // Edge case
    assert_eq!(decimal_length17(v), 1);
}

