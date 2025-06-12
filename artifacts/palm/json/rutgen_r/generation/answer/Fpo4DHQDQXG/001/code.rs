// Answer 0

#[test]
fn test_is_array_with_array_value() {
    use serde_json::json;

    let array_value = json!(["value1", "value2", "value3"]);
    assert!(array_value.is_array());
}

#[test]
fn test_is_array_with_empty_array() {
    use serde_json::json;

    let empty_array_value = json!([]);
    assert!(empty_array_value.is_array());
}

#[test]
fn test_is_array_with_object_value() {
    use serde_json::json;

    let object_value = json!({"key": "value"});
    assert!(!object_value.is_array());
}

#[test]
fn test_is_array_with_string_value() {
    use serde_json::json;

    let string_value = json!("just a string");
    assert!(!string_value.is_array());
}

#[test]
fn test_is_array_with_number_value() {
    use serde_json::json;

    let number_value = json!(42);
    assert!(!number_value.is_array());
}

#[test]
fn test_is_array_with_boolean_value() {
    use serde_json::json;

    let boolean_value_true = json!(true);
    assert!(!boolean_value_true.is_array());

    let boolean_value_false = json!(false);
    assert!(!boolean_value_false.is_array());
}

#[test]
fn test_is_array_with_null_value() {
    use serde_json::json;

    let null_value = json!(null);
    assert!(!null_value.is_array());
}

#[test]
fn test_is_array_with_nested_array() {
    use serde_json::json;

    let nested_array_value = json!([["nested1", "nested2"], ["nested3", "nested4"]]);
    assert!(nested_array_value.is_array());
}

