// Answer 0

#[test]
fn test_is_i64_with_min_i64() {
    let value = Value::Number(Number::from_i128(i128::MIN).unwrap());
    value.is_i64();
}

#[test]
fn test_is_i64_with_max_i64() {
    let value = Value::Number(Number::from_i128(i128::MAX).unwrap());
    value.is_i64();
}

#[test]
fn test_is_i64_with_negative_value() {
    let value = Value::Number(Number::from_i128(-1).unwrap());
    value.is_i64();
}

#[test]
fn test_is_i64_with_zero() {
    let value = Value::Number(Number::from_i128(0).unwrap());
    value.is_i64();
}

#[test]
fn test_is_i64_with_a_large_positive_integer() {
    let value = Value::Number(Number::from_i128(123456789).unwrap());
    value.is_i64();
}

#[test]
fn test_is_i64_with_a_large_negative_integer() {
    let value = Value::Number(Number::from_i128(-123456789).unwrap());
    value.is_i64();
}

