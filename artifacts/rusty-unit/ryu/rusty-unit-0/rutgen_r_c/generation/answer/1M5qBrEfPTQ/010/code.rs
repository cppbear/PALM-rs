// Answer 0

#[test]
fn test_decimal_length17_value_equals_10000000() {
    let v: u64 = 10000000;
    let result = decimal_length17(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length17_value_equals_9999999() {
    let v: u64 = 9999999;
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_value_equals_1000000() {
    let v: u64 = 1000000;
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_value_equals_999999() {
    let v: u64 = 999999;
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length17_value_equals_100000() {
    let v: u64 = 100000;
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length17_value_equals_99999() {
    let v: u64 = 99999;
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_value_equals_10000() {
    let v: u64 = 10000;
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_value_equals_9999() {
    let v: u64 = 9999;
    let result = decimal_length17(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length17_value_equals_1000() {
    let v: u64 = 1000;
    let result = decimal_length17(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length17_value_equals_999() {
    let v: u64 = 999;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_value_equals_100() {
    let v: u64 = 100;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_value_equals_9() {
    let v: u64 = 9;
    let result = decimal_length17(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length17_value_equals_1() {
    let v: u64 = 1;
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

