// Answer 0

#[test]
fn test_decimal_length17_lower_bound() {
    let v: u64 = 1000;
    let result = decimal_length17(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length17_boundary_case() {
    let v: u64 = 999;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_high_bounds() {
    let v: u64 = 99999999999999999; // just below 100000000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

#[test]
#[should_panic]
fn test_decimal_length17_above_upper_bound() {
    let v: u64 = 100000000000000000; // triggers debug assert
    decimal_length17(v);
}

