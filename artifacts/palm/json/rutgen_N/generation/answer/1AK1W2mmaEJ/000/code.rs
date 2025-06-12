// Answer 0

#[test]
fn test_as_array_mut_with_array_value() {
    use serde_json::{json, Value};

    let mut v = json!({ "a": ["an", "array"] });

    // Call as_array_mut and clear the array
    v["a"].as_array_mut().unwrap().clear();
    assert_eq!(v, json!({ "a": [] }));
}

#[test]
fn test_as_array_mut_with_non_array_value() {
    use serde_json::{json, Value};

    let mut v = json!({ "a": "not an array" });

    // Call as_array_mut and expect None
    assert!(v["a"].as_array_mut().is_none());
}

#[test]
fn test_as_array_mut_with_empty_array() {
    use serde_json::{json, Value};

    let mut v = json!({ "a": [] });

    // Call as_array_mut and clear the array
    v["a"].as_array_mut().unwrap().clear();
    assert_eq!(v, json!({ "a": [] }));
}

