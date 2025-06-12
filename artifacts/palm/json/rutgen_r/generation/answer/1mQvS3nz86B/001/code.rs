// Answer 0

#[test]
fn test_deserialize_bool() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("true"));
    assert_eq!(deserialized, Ok(Value::Bool(true)));
}

#[test]
fn test_deserialize_i64() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("123"));
    assert_eq!(deserialized, Ok(Value::Number(123.into())));
}

#[test]
fn test_deserialize_i128() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("9223372036854775807"));
    assert!(deserialized.is_ok());
}

#[test]
fn test_deserialize_u64() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("12345678901234"));
    assert_eq!(deserialized, Ok(Value::Number(12345678901234u64.into())));
}

#[test]
fn test_deserialize_u128() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("18446744073709551615"));
    assert!(deserialized.is_ok());
}

#[test]
fn test_deserialize_f64() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("3.14"));
    assert!(matches!(deserialized, Ok(Value::Number(_))));
}

#[test]
fn test_deserialize_string() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("\"hello\""));
    assert_eq!(deserialized, Ok(Value::String("hello".to_string())));
}

#[test]
fn test_deserialize_none() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("null"));
    assert_eq!(deserialized, Ok(Value::Null));
}

#[test]
fn test_deserialize_some() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("\"some_value\""));
    assert_eq!(deserialized, Ok(Value::String("some_value".to_string())));
}

#[test]
fn test_deserialize_unit() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("null"));
    assert_eq!(deserialized, Ok(Value::Null));
}

#[test]
fn test_deserialize_array() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("[1, 2, 3]"));
    assert!(matches!(deserialized, Ok(Value::Array(_))));
}

#[test]
fn test_deserialize_map() {
    let deserialized: Result<Value, _> = deserialize(serde_json::Deserializer::from_str("{\"key\": \"value\"}"));
    assert!(matches!(deserialized, Ok(Value::Object(_))));
}

