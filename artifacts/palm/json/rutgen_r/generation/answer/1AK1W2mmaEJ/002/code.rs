// Answer 0

#[test]
fn test_as_array_mut_with_empty_array() {
    use serde_json::Value;
    let mut value = Value::Array(vec![]);
    let result = value.as_array_mut();
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_as_array_mut_with_non_empty_array() {
    use serde_json::Value;
    let mut value = Value::Array(vec![Value::String("item".to_string()), Value::Number(1.into())]);
    let result = value.as_array_mut();
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_as_array_mut_with_null() {
    use serde_json::Value;
    let mut value = Value::Null;
    let result = value.as_array_mut();
    assert!(result.is_none());
}

#[test]
fn test_as_array_mut_with_object() {
    use serde_json::Value;
    let mut value = Value::Object(serde_json::Map::new());
    let result = value.as_array_mut();
    assert!(result.is_none());
}

#[test]
fn test_as_array_mut_with_nested_array() {
    use serde_json::Value;
    let mut value = Value::Array(vec![Value::Array(vec![Value::String("nested".to_string())])]);
    let result = value.as_array_mut();
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 1);
}

