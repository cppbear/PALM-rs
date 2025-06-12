// Answer 0

#[test]
fn test_is_i64_positive_integer_within_bounds() {
    let num = Number::from(42u64);
    assert!(num.is_i64());
}

#[test]
fn test_is_i64_negative_integer_within_bounds() {
    let num = Number::from(-42i64);
    assert!(num.is_i64());
}

#[test]
fn test_is_i64_positive_integer_out_of_bounds() {
    let num = Number::from(18446744073709551615u128); // max u128
    assert!(!num.is_i64());
}

#[test]
fn test_is_i64_negative_integer_out_of_bounds() {
    let num = Number::from(-9223372036854775809i128); // out of bounds
    assert!(!num.is_i64());
}

#[test]
fn test_is_i64_float_value() {
    let num = Number::from(3.14f64);
    assert!(!num.is_i64());
}

#[test]
fn test_is_i64_zero() {
    let num = Number::from(0i64);
    assert!(num.is_i64());
}

