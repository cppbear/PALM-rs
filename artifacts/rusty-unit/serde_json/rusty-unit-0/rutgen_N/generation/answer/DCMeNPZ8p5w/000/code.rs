// Answer 0

#[test]
fn test_as_array_with_array_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({"a": ["an", "array"], "b": {"an": "object"}});
    
    assert_eq!(v["a"].as_array().unwrap().len(), 2);
}

#[test]
fn test_as_array_with_non_array_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({"a": ["an", "array"], "b": {"an": "object"}});

    assert_eq!(v["b"].as_array(), None);
}

#[test]
fn test_as_array_with_empty_array() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({"empty": []});

    assert_eq!(v["empty"].as_array().unwrap().len(), 0);
}

#[test]
fn test_as_array_with_non_value() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(null);

    assert_eq!(v.as_array(), None);
}

