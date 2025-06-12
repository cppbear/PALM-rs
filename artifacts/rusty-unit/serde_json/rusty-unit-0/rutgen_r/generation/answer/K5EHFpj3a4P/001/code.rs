// Answer 0

#[test]
fn test_as_bool_with_non_boolean_value() {
    use serde_json::Value;

    let non_bool_value = Value::String("false".to_string());
    assert_eq!(non_bool_value.as_bool(), None);

    let another_non_bool_value = Value::Number(serde_json::Number::from(0));
    assert_eq!(another_non_bool_value.as_bool(), None);

    let yet_another_non_bool_value = Value::Array(vec![Value::Bool(true)]);
    assert_eq!(yet_another_non_bool_value.as_bool(), None);

    let an_empty_object_value = Value::Object(serde_json::Map::new());
    assert_eq!(an_empty_object_value.as_bool(), None);
}

#[test]
fn test_as_bool_with_null_value() {
    use serde_json::Value;

    let null_value = Value::Null;
    assert_eq!(null_value.as_bool(), None);
}

