// Answer 0

#[test]
fn test_decimal_length17_with_value_0() {
    assert_eq!(decimal_length17(0), 1);
}

#[test]
fn test_decimal_length17_with_value_1() {
    assert_eq!(decimal_length17(1), 1);
}

#[test]
fn test_decimal_length17_with_value_9() {
    assert_eq!(decimal_length17(9), 1);
}

#[test]
fn test_decimal_length17_with_value_10() {
    assert_eq!(decimal_length17(10), 2);
}

#[test]
fn test_decimal_length17_with_value_99() {
    assert_eq!(decimal_length17(99), 2);
}

#[test]
fn test_decimal_length17_with_value_100() {
    assert_eq!(decimal_length17(100), 3);
}

#[test]
fn test_decimal_length17_with_value_999() {
    assert_eq!(decimal_length17(999), 3);
}

#[test]
fn test_decimal_length17_with_value_1000() {
    assert_eq!(decimal_length17(1000), 4);
}

#[test]
fn test_decimal_length17_with_value_9999() {
    assert_eq!(decimal_length17(9999), 4);
}

#[test]
fn test_decimal_length17_with_value_10000() {
    assert_eq!(decimal_length17(10000), 5);
}

