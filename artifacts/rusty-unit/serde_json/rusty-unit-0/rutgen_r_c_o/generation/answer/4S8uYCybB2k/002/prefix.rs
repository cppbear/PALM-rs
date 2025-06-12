// Answer 0

#[test]
fn test_as_u64_valid_case() {
    let value = Value::Number(Number::from_u128(0).unwrap());
    value.as_u64();
}

#[test]
fn test_as_u64_valid_case_large() {
    let value = Value::Number(Number::from_u128(u64::MAX as u128).unwrap());
    value.as_u64();
}

#[test]
fn test_as_u64_invalid_case_negative() {
    let value = Value::Number(Number::from_i128(-1).unwrap());
    value.as_u64();
}

#[test]
fn test_as_u64_invalid_case_float() {
    let value = Value::Number(Number::from_f64(256.5).unwrap());
    value.as_u64();
}

#[test]
fn test_as_u64_invalid_case_zero_floating() {
    let value = Value::Number(Number::from_f64(0.0).unwrap());
    value.as_u64();
}

#[test]
fn test_as_u64_case_large_neg() {
    let value = Value::Number(Number::from_i128(-10).unwrap());
    value.as_u64();
}

