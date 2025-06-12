// Answer 0

#[test]
pub fn test_decimal_length17_minimum_value() {
    let v: u64 = 1;
    decimal_length17(v);
}

#[test]
pub fn test_decimal_length17_small_value() {
    let v: u64 = 10;
    decimal_length17(v);
}

#[test]
pub fn test_decimal_length17_medium_value() {
    let v: u64 = 100000;
    decimal_length17(v);
}

#[test]
pub fn test_decimal_length17_large_value() {
    let v: u64 = 99999999;
    decimal_length17(v);
}

#[test]
#[should_panic]
pub fn test_decimal_length17_exceed_upper_bound() {
    let v: u64 = 100000000000000000;
    decimal_length17(v);
}

