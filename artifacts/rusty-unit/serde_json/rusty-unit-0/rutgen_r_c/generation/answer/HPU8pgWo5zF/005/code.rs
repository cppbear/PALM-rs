// Answer 0

#[test]
fn test_serialize_array() {
    use serde_json::Value;

    let array_value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let serialized = serde_json::to_string(&array_value).unwrap();
    assert_eq!(serialized, "[true,false]");

    let empty_array_value = Value::Array(vec![]);
    let serialized_empty = serde_json::to_string(&empty_array_value).unwrap();
    assert_eq!(serialized_empty, "[]");
}

#[test]
fn test_serialize_nested_array() {
    use serde_json::Value;

    let nested_array_value = Value::Array(vec![
        Value::Array(vec![Value::Number(serde_json::Number::from(1))]),
        Value::Array(vec![Value::Number(serde_json::Number::from(2)), Value::Number(serde_json::Number::from(3))]),
    ]);
    let serialized_nested = serde_json::to_string(&nested_array_value).unwrap();
    assert_eq!(serialized_nested, "[[1],[2,3]]");
}

#[test]
fn test_serialize_mixed_types_in_array() {
    use serde_json::Value;

    let mixed_array_value = Value::Array(vec![
        Value::Bool(true),
        Value::String("test".to_string()),
        Value::Number(serde_json::Number::from(42)),
    ]);
    let serialized_mixed = serde_json::to_string(&mixed_array_value).unwrap();
    assert_eq!(serialized_mixed, "[true,\"test\",42]");
}

