// Answer 0

#[test]
fn test_decimal_length17_lower_bound() {
    let v: u64 = 100_000_000_000;
    let result = decimal_length17(v);
    assert_eq!(result, 12);
}

#[test]
fn test_decimal_length17_mid_range() {
    let v: u64 = 1_000_000_000_000;
    let result = decimal_length17(v);
    assert_eq!(result, 13);
}

#[test]
fn test_decimal_length17_high_range() {
    let v: u64 = 10_000_000_000_000_000;
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

#[test]
#[should_panic]
fn test_decimal_length17_upper_bound() {
    let v: u64 = 100_000_000_000_000_000; // This will trigger a panic due to the debug assertion.
    decimal_length17(v);
}

