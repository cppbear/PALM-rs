// Answer 0

#[test]
fn test_is_i64_positive_int_zero() {
    let number = Number::from(0u64);
    let result = number.is_i64();
}

#[test]
fn test_is_i64_positive_int_max() {
    let number = Number::from(9223372036854775807u64);
    let result = number.is_i64();
}

#[test]
fn test_is_i64_positive_int_within_range() {
    let number = Number::from(1234567890123456789u64);
    let result = number.is_i64();
}

#[test]
fn test_is_i64_negative_int() {
    let number = Number::from(-1i64);
    let result = number.is_i64();
}

#[test]
fn test_is_i64_float() {
    let number = Number::from(3.14f64);
    let result = number.is_i64();
} 

#[test]
fn test_is_i64_large_positive_int() {
    let number = Number::from(9223372036854775806u64);
    let result = number.is_i64();
}

#[test]
fn test_is_i64_large_float() {
    let number = Number::from(1.7976931348623157e+308f64);
    let result = number.is_i64();
}

