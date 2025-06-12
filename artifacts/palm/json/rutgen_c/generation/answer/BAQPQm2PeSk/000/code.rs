// Answer 0

#[test]
fn test_is_object_with_object_value() {
    use serde_json::Value;

    let obj = Value::Object(Map::new());
    assert!(obj.is_object());
}

#[test]
fn test_is_object_with_array_value() {
    use serde_json::Value;

    let array_value = Value::Array(vec![Value::String("an array".to_string())]);
    assert!(!array_value.is_object());
}

#[test]
fn test_is_object_with_string_value() {
    use serde_json::Value;

    let string_value = Value::String("a string".to_string());
    assert!(!string_value.is_object());
}

#[test]
fn test_is_object_with_number_value() {
    use serde_json::Value;

    let number_value = Value::Number(Number { n: 42 });
    assert!(!number_value.is_object());
}

#[test]
fn test_is_object_with_boolean_value() {
    use serde_json::Value;

    let boolean_value = Value::Bool(true);
    assert!(!boolean_value.is_object());
}

#[test]
fn test_is_object_with_null_value() {
    use serde_json::Value;

    let null_value = Value::Null;
    assert!(!null_value.is_object());
}

