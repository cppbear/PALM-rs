// Answer 0

#[test]
fn test_decimal_length17_lower_bound() {
    let v = 1000000000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_mid_range() {
    let v = 5000000000000;
    decimal_length17(v);
}

#[test]
fn test_decimal_length17_upper_bound() {
    let v = 9999999999999;
    decimal_length17(v);
}

#[test]
#[should_panic]
fn test_decimal_length17_out_of_bounds() {
    let v = 10000000000000;
    decimal_length17(v);
}

