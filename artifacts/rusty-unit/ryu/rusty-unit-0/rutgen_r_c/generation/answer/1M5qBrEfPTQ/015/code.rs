// Answer 0

#[test]
fn test_decimal_length17_lower_bound() {
    let v: u64 = 100; // satisfies v < 100000000000000000 and v >= 100
    assert_eq!(decimal_length17(v), 3);
}

#[test]
fn test_decimal_length17_between_bounds() {
    let v: u64 = 999; // satisfies v < 100000000000000000 and v < 1000
    assert_eq!(decimal_length17(v), 3);
}

#[test]
fn test_decimal_length17_higher_value() {
    let v: u64 = 9999; // satisfies v < 100000000000000000 and v < 10000
    assert_eq!(decimal_length17(v), 4);
}

#[test]
fn test_decimal_length17_exactly_10() {
    let v: u64 = 10_000_000; // satisfies v < 100000000000000000 and v >= 10000000
    assert_eq!(decimal_length17(v), 8);
}

#[test]
fn test_decimal_length17_high_value() {
    let v: u64 = 99_999_999_999_999_999; // maximum value just under the limit
    assert_eq!(decimal_length17(v), 17);
}

