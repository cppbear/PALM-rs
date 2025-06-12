// Answer 0

#[test]
fn test_deserialize_bool() {
    let json = serde_json::json!(true);
    let result: Result<Value, _> = deserialize(json);
    assert_eq!(result, Ok(Value::Bool(true)));
}

#[test]
fn test_deserialize_i64() {
    let json = serde_json::json!(42);
    let result: Result<Value, _> = deserialize(json);
    assert_eq!(result, Ok(Value::Number(Number::from(42))));
}

#[test]
fn test_deserialize_u64() {
    let json = serde_json::json!(42u64);
    let result: Result<Value, _> = deserialize(json);
    assert_eq!(result, Ok(Value::Number(Number::from(42))));
}

#[test]
fn test_deserialize_f64() {
    let json = serde_json::json!(3.14);
    let result: Result<Value, _> = deserialize(json);
    assert_eq!(result, Ok(Value::Number(Number::from_f64(3.14).unwrap())));
}

#[test]
fn test_deserialize_string() {
    let json = serde_json::json!("Hello, world!");
    let result: Result<Value, _> = deserialize(json);
    assert_eq!(result, Ok(Value::String("Hello, world!".to_string())));
}

#[test]
fn test_deserialize_null() {
    let json = serde_json::json!(null);
    let result: Result<Value, _> = deserialize(json);
    assert_eq!(result, Ok(Value::Null));
}

#[test]
fn test_deserialize_array() {
    let json = serde_json::json!([1, 2, 3]);
    let result: Result<Value, _> = deserialize(json);
    let expected = Value::Array(vec![
        Value::Number(Number::from(1)),
        Value::Number(Number::from(2)),
        Value::Number(Number::from(3)),
    ]);
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_deserialize_object() {
    let json = serde_json::json!({"key": "value"});
    let result: Result<Value, _> = deserialize(json);
    let mut expected = Map::new();
    expected.insert(Value::String("key".to_string()), Value::String("value".to_string()));
    assert_eq!(result, Ok(Value::Object(expected)));
}

