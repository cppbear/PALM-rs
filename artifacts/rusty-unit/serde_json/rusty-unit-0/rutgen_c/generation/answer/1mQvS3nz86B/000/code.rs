// Answer 0

#[test]
fn test_deserialize_bool() {
    use serde_json::from_str;

    let result: Result<Value, _> = from_str("true");
    assert_eq!(result, Ok(Value::Bool(true)));

    let result: Result<Value, _> = from_str("false");
    assert_eq!(result, Ok(Value::Bool(false)));
}

#[test]
fn test_deserialize_i64() {
    use serde_json::from_str;

    let result: Result<Value, _> = from_str("42");
    assert_eq!(result, Ok(Value::Number(Number { n: 42.into() })));
}

#[test]
fn test_deserialize_u64() {
    use serde_json::from_str;

    let result: Result<Value, _> = from_str("4000000000");
    assert_eq!(result, Ok(Value::Number(Number { n: 4000000000u64.into() })));
}

#[test]
fn test_deserialize_f64() {
    use serde_json::from_str;

    let result: Result<Value, _> = from_str("3.14");
    assert!(matches!(result, Ok(Value::Number(Number { n }) if n == Number::from_f64(3.14).unwrap())));
}

#[test]
fn test_deserialize_string() {
    use serde_json::from_str;

    let result: Result<Value, _> = from_str("\"Hello, world!\"");
    assert_eq!(result, Ok(Value::String("Hello, world!".to_string())));
}

#[test]
fn test_deserialize_array() {
    use serde_json::from_str;

    let result: Result<Value, _> = from_str("[1, 2, 3]");
    assert_eq!(result, Ok(Value::Array(vec![
        Value::Number(Number { n: 1.into() }),
        Value::Number(Number { n: 2.into() }),
        Value::Number(Number { n: 3.into() }),
    ])));
}

#[test]
fn test_deserialize_object() {
    use serde_json::from_str;

    let result: Result<Value, _> = from_str("{\"key\": \"value\"}");
    let mut expected_map = Map::new();
    expected_map.insert("key".to_string(), Value::String("value".to_string()));
    assert_eq!(result, Ok(Value::Object(expected_map)));
}

