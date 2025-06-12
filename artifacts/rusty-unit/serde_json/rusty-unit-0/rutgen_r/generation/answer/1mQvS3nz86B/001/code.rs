// Answer 0

#[test]
fn test_deserialize_bool() {
    let json = "true";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Bool(true)));
}

#[test]
fn test_deserialize_i64() {
    let json = "42";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Number(Number::from(42))));
}

#[test]
fn test_deserialize_i128() {
    let json = "9223372036854775807"; // Maximum i64 value
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Number(Number::from(9223372036854775807_i128))));
}

#[test]
fn test_deserialize_u64() {
    let json = "42";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Number(Number::from(42_u64))));
}

#[test]
fn test_deserialize_u128() {
    let json = "18446744073709551615"; // Maximum u64 value
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Number(Number::from(18446744073709551615_u128))));
}

#[test]
fn test_deserialize_f64() {
    let json = "3.14";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Number(Number::from_f64(3.14).unwrap())));
}

#[test]
fn test_deserialize_string() {
    let json = "\"hello\"";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::String(String::from("hello"))));
}

#[test]
fn test_deserialize_none() {
    let json = "null";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Null));
}

#[test]
fn test_deserialize_sequence() {
    let json = "[1, 2, 3]";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Array(vec![
        Value::Number(Number::from(1)),
        Value::Number(Number::from(2)),
        Value::Number(Number::from(3)),
    ])));
}

#[test]
fn test_deserialize_empty_sequence() {
    let json = "[]";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Array(vec![])));
}

#[test]
fn test_deserialize_object() {
    let json = "{\"key\":\"value\"}";
    let mut expected_map = Map::new();
    expected_map.insert(Value::String(String::from("key")), Value::String(String::from("value")));

    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Object(expected_map)));
}

#[test]
fn test_deserialize_empty_object() {
    let json = "{}";
    let result: Result<Value, _> = serde_json::from_str(json);
    assert_eq!(result, Ok(Value::Object(Map::new())));
}

#[should_panic]
fn test_deserialize_invalid_json() {
    let json = "invalid json";
    let _result: Result<Value, _> = serde_json::from_str(json);
}

