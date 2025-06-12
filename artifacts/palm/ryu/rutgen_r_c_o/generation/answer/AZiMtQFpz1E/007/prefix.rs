// Answer 0

#[test]
fn test_decimal_length9_lower_bound() {
    let v = 100;
    decimal_length9(v);
}

#[test]
fn test_decimal_length9_middle_value() {
    let v = 500;
    decimal_length9(v);
}

#[test]
fn test_decimal_length9_upper_bound() {
    let v = 999;
    decimal_length9(v);
}

