// Answer 0

#[test]
fn test_serialize_i128_positive_value() {
    let value: i128 = 12345678901234567890;
    let result = serialize_i128(value);
    assert_eq!(result, Ok("12345678901234567890".to_string()));
}

#[test]
fn test_serialize_i128_negative_value() {
    let value: i128 = -12345678901234567890;
    let result = serialize_i128(value);
    assert_eq!(result, Ok("-12345678901234567890".to_string()));
}

#[test]
fn test_serialize_i128_zero() {
    let value: i128 = 0;
    let result = serialize_i128(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_i128_min_value() {
    let value: i128 = i128::MIN;
    let result = serialize_i128(value);
    // Since the return value is determined by itoa, we will check the expected string representation of i128::MIN
    assert_eq!(result, Ok("-170141183460469231731687303715884105728".to_string()));
}

#[test]
fn test_serialize_i128_max_value() {
    let value: i128 = i128::MAX;
    let result = serialize_i128(value);
    // Similar to min value, we need to check the expected representation of i128::MAX
    assert_eq!(result, Ok("170141183460469231731687303715884105727".to_string()));
}

