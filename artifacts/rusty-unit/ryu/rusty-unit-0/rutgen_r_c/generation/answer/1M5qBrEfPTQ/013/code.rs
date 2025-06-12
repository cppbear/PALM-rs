// Answer 0

#[test]
fn test_decimal_length17_min_value() {
    let v: u64 = 1;
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

#[test]
fn test_decimal_length17_boundary_value() {
    let v: u64 = 10000;
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_low_value() {
    let v: u64 = 9999;
    let result = decimal_length17(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length17_high_value() {
    let v: u64 = 99999;
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_large_value() {
    let v: u64 = 99999999999999999; // Just under the limit, should count as 17
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

