// Answer 0

#[test]
fn test_deserialize_i64() {
    use serde_json::de::from_value;
    use serde_json::Value;

    let json_value = Value::Number(serde_json::Number::from(42));
    let result: Result<Number, _> = from_value(json_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from(42));
}

#[test]
fn test_deserialize_u64() {
    use serde_json::de::from_value;
    use serde_json::Value;

    let json_value = Value::Number(serde_json::Number::from(42u64));
    let result: Result<Number, _> = from_value(json_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from(42u64));
}

#[test]
fn test_deserialize_i128() {
    use serde_json::de::from_value;
    use serde_json::Value;

    let json_value = Value::Number(serde_json::Number::from(123456789012345678901234567890i128));
    let result: Result<Number, _> = from_value(json_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_i128(123456789012345678901234567890).unwrap());
}

#[test]
fn test_deserialize_u128() {
    use serde_json::de::from_value;
    use serde_json::Value;

    let json_value = Value::Number(serde_json::Number::from(123456789012345678901234567890u128));
    let result: Result<Number, _> = from_value(json_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_u128(123456789012345678901234567890).unwrap());
}

#[test]
fn test_deserialize_f64() {
    use serde_json::de::from_value;
    use serde_json::Value;

    let json_value = Value::Number(serde_json::Number::from_f64(42.0).unwrap());
    let result: Result<Number, _> = from_value(json_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::from_f64(42.0).unwrap());
}

#[test]
#[should_panic(expected = "JSON number out of range")]
fn test_deserialize_invalid_i128() {
    use serde_json::de::from_value;
    use serde_json::Value;

    let json_value = Value::Number(serde_json::Number::from(u128::MAX + 1));
    let _result: Result<Number, _> = from_value(json_value).unwrap();
}

#[test]
#[should_panic(expected = "not a JSON number")]
fn test_deserialize_invalid_f64() {
    use serde_json::de::from_value;
    use serde_json::Value;

    let json_value = Value::String("not_a_number".to_string());
    let _result: Result<Number, _> = from_value(json_value).unwrap();
}

