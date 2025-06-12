// Answer 0

#[test]
fn test_is_array_with_empty_array() {
    use serde_json::json;

    let value = json!([]);
    assert!(value.is_array());
}

#[test]
fn test_is_array_with_non_array_object() {
    use serde_json::json;

    let value = json!({ "key": "value" });
    assert!(!value.is_array());
}

#[test]
fn test_is_array_with_null() {
    use serde_json::json;

    let value = json!(null);
    assert!(!value.is_array());
}

#[test]
fn test_is_array_with_single_element_array() {
    use serde_json::json;

    let value = json!(["element"]);
    assert!(value.is_array());
}

#[test]
fn test_is_array_with_nested_arrays() {
    use serde_json::json;

    let value = json!([["nested", "array"]]);
    assert!(value.is_array());
}

#[test]
fn test_is_array_with_complex_object() {
    use serde_json::json;

    let value = json!({ "array": [1, 2, 3], "object": { "key": "value" } });
    assert!(value["array"].is_array());
    assert!(!value["object"].is_array());
}

#[test]
fn test_is_array_with_integer() {
    use serde_json::json;

    let value = json!(42);
    assert!(!value.is_array());
}

#[test]
fn test_is_array_with_boolean() {
    use serde_json::json;

    let value = json!(true);
    assert!(!value.is_array());
}

#[test]
fn test_is_array_with_float() {
    use serde_json::json;

    let value = json!(3.14);
    assert!(!value.is_array());
}

