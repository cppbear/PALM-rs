// Answer 0

#[test]
fn test_as_i64_valid_min() {
    let number = Number::from_i128(i64::MIN as i128).unwrap();
    let value = Value::Number(number);
    let result = value.as_i64();
}

#[test]
fn test_as_i64_valid_zero() {
    let number = Number::from_i128(0).unwrap();
    let value = Value::Number(number);
    let result = value.as_i64();
}

#[test]
fn test_as_i64_valid_max() {
    let number = Number::from_i128(i64::MAX as i128).unwrap();
    let value = Value::Number(number);
    let result = value.as_i64();
}

#[test]
fn test_as_i64_valid_positive() {
    let number = Number::from_i128(123).unwrap();
    let value = Value::Number(number);
    let result = value.as_i64();
}

#[test]
fn test_as_i64_valid_negative() {
    let number = Number::from_i128(-123).unwrap();
    let value = Value::Number(number);
    let result = value.as_i64();
}

#[test]
fn test_as_i64_invalid_overflow() {
    let number = Number::from_f64(i64::MAX as f64 + 1.0).unwrap();
    let value = Value::Number(number);
    let result = value.as_i64();
}

#[test]
fn test_as_i64_invalid_float() {
    let number = Number::from_f64(3.14).unwrap();
    let value = Value::Number(number);
    let result = value.as_i64();
}

#[test]
fn test_as_i64_invalid_non_number() {
    let value = Value::Null;
    let result = value.as_i64();
}

