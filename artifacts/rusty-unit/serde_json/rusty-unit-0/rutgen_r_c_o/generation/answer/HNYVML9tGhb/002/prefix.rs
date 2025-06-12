// Answer 0

#[test]
fn test_is_u64_valid_small() {
    let value = Value::Number(Number::from_u128(0).unwrap());
    value.is_u64();
}

#[test]
fn test_is_u64_valid_large() {
    let value = Value::Number(Number::from_u128(18446744073709551615).unwrap());
    value.is_u64();
}

#[test]
fn test_is_u64_valid_mid_range() {
    let value = Value::Number(Number::from_u128(123456789).unwrap());
    value.is_u64();
}

#[test]
fn test_is_u64_negative() {
    let value = Value::Number(Number::from_i128(-1).unwrap());
    value.is_u64();
}

#[test]
fn test_is_u64_valid_float() {
    let value = Value::Number(Number::from_f64(256.0).unwrap());
    value.is_u64();
}

#[test]
fn test_is_u64_overflow() {
    let value = Value::Number(Number::from_u128(18446744073709551616).unwrap());
    value.is_u64();
}

#[test]
fn test_is_u64_non_number_value() {
    let value = Value::Bool(true);
    value.is_u64();
}

#[test]
fn test_is_u64_null_value() {
    let value = Value::Null;
    value.is_u64();
}

#[test]
fn test_is_u64_string_value() {
    let value = Value::String(String::from("a string"));
    value.is_u64();
}

