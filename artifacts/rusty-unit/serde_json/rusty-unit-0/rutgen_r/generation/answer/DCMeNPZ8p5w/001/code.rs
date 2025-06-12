// Answer 0

#[test]
fn test_as_array_with_object() {
    use serde_json::{json, Value};

    let v = json!({ "key": "value" });

    assert_eq!(v.as_array(), None);
}

#[test]
fn test_as_array_with_number() {
    use serde_json::{json, Value};

    let v = json!(42);

    assert_eq!(v.as_array(), None);
}

#[test]
fn test_as_array_with_string() {
    use serde_json::{json, Value};

    let v = json!("string");

    assert_eq!(v.as_array(), None);
}

#[test]
fn test_as_array_with_boolean() {
    use serde_json::{json, Value};

    let v = json!(true);

    assert_eq!(v.as_array(), None);
}

#[test]
fn test_as_array_with_null() {
    use serde_json::{json, Value};

    let v = json!(null);

    assert_eq!(v.as_array(), None);
}

#[test]
fn test_as_array_with_object_array() {
    use serde_json::{json, Value};

    let v = json!([{"key": "value"}, {"another_key": "another_value"}]);

    assert_eq!(v.as_array(), Some(&v.as_array().unwrap())); // Covers an empty array as valid input, which returns self
}

