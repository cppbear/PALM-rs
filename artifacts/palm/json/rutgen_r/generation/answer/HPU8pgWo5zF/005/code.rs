// Answer 0

#[test]
fn test_serialize_array_empty() {
    let value = serde_json::Value::Array(vec![]);
    let result = serde_json::to_string(&value);
    assert_eq!(result, Ok("[]".to_string()));
}

#[test]
fn test_serialize_array_single_element() {
    let value = serde_json::Value::Array(vec![serde_json::Value::Number(serde_json::Number::from(42))]);
    let result = serde_json::to_string(&value);
    assert_eq!(result, Ok("[42]".to_string()));
}

#[test]
fn test_serialize_array_multiple_elements() {
    let value = serde_json::Value::Array(vec![
        serde_json::Value::Bool(true),
        serde_json::Value::Null,
        serde_json::Value::String("Hello".to_string()),
        serde_json::Value::Number(serde_json::Number::from(3.14)),
    ]);
    let result = serde_json::to_string(&value);
    assert_eq!(result, Ok("[true,null,\"Hello\",3.14]".to_string()));
}

#[test]
fn test_serialize_array_nested() {
    let value = serde_json::Value::Array(vec![
        serde_json::Value::Array(vec![
            serde_json::Value::Number(serde_json::Number::from(1)),
            serde_json::Value::Number(serde_json::Number::from(2)),
        ]),
        serde_json::Value::Array(vec![
            serde_json::Value::Bool(false),
            serde_json::Value::String("Test".to_string()),
        ]),
    ]);
    let result = serde_json::to_string(&value);
    assert_eq!(result, Ok("[[1,2],[false,\"Test\"]]".to_string()));
}

