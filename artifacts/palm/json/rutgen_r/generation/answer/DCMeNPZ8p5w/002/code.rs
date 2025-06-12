// Answer 0

#[test]
fn test_as_array_with_non_empty_array() {
    use serde_json::json;
    let v = json!({ "a": ["first", "second", "third"] });
    assert_eq!(v["a"].as_array().unwrap().len(), 3);
}

#[test]
fn test_as_array_with_empty_array() {
    use serde_json::json;
    let v = json!({ "a": [] });
    assert_eq!(v["a"].as_array().unwrap().len(), 0);
}

#[test]
fn test_as_array_with_nested_array() {
    use serde_json::json;
    let v = json!({ "a": [["nested", "array"]] });
    assert_eq!(v["a"].as_array().unwrap().len(), 1);
    assert_eq!(v["a"].as_array().unwrap()[0].as_array().unwrap().len(), 2);
}

#[test]
fn test_as_array_with_mixed_types() {
    use serde_json::json;
    let v = json!({ "a": [1, "two", true] });
    assert_eq!(v["a"].as_array().unwrap().len(), 3);
}

#[test]
fn test_as_array_with_non_array_value() {
    use serde_json::json;
    let v = json!({ "b": { "non": "array" } });
    assert_eq!(v["b"].as_array(), None);
}

