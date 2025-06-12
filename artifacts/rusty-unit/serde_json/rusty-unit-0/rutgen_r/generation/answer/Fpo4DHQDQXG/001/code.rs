// Answer 0

#[test]
fn test_is_array_true() {
    let array_value = serde_json::json!([1, 2, 3]);
    assert!(array_value.is_array());
}

#[test]
fn test_is_array_empty() {
    let empty_array_value = serde_json::json!([]);
    assert!(empty_array_value.is_array());
}

#[test]
fn test_is_array_false_object() {
    let object_value = serde_json::json!({ "key": "value" });
    assert!(!object_value.is_array());
}

#[test]
fn test_is_array_false_string() {
    let string_value = serde_json::json!("just a string");
    assert!(!string_value.is_array());
}

#[test]
fn test_is_array_false_number() {
    let number_value = serde_json::json!(42);
    assert!(!number_value.is_array());
}

#[test]
fn test_is_array_false_boolean() {
    let boolean_value_true = serde_json::json!(true);
    let boolean_value_false = serde_json::json!(false);
    assert!(!boolean_value_true.is_array());
    assert!(!boolean_value_false.is_array());
}

#[test]
fn test_is_array_nested() {
    let nested_value = serde_json::json!([1, { "inner_key": "inner_value" }, 3]);
    assert!(nested_value.is_array());
}

#[test]
fn test_is_array_null() {
    let null_value = serde_json::json!(null);
    assert!(!null_value.is_array());
}

