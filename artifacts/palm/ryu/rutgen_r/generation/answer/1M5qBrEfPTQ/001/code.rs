// Answer 0

#[test]
fn test_decimal_length17_upper_boundary() {
    let v = 10000000000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

#[test]
fn test_decimal_length17_exclusive_upper_boundary() {
    let v = 99999999999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

#[test]
fn test_decimal_length17_lower_boundary() {
    let v = 1000000000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 16);
}

#[test]
fn test_decimal_length17_valid_case() {
    let v = 50000000000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

