// Answer 0

#[test]
fn test_pow5_factor_small_value() {
    let value = 1;
    pow5_factor(value);
}

#[test]
fn test_pow5_factor_large_value_near_upper_bound() {
    let value = 18446744073709551615;
    pow5_factor(value);
}

#[test]
fn test_pow5_factor_mid_range_value() {
    let value = 1000000000000000;
    pow5_factor(value);
}

#[test]
fn test_pow5_factor_even_large_value() {
    let value = 123456789012345678;
    pow5_factor(value);
}

#[should_panic]
fn test_pow5_factor_zero_value() {
    let value = 0;
    pow5_factor(value);
}

