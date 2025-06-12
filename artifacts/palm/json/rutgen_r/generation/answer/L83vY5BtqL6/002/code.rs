// Answer 0

#[test]
fn test_as_object_mut_with_valid_object() {
    use serde_json::{json, Value, Map};

    let mut v = json!({ "key1": "value1", "key2": "value2" });

    // Should be a mutable reference to the map
    let map = v.as_object_mut().unwrap();
    map.insert("key3".to_string(), json!("value3"));

    assert_eq!(v, json!({ "key1": "value1", "key2": "value2", "key3": "value3" }));
}

#[test]
fn test_as_object_mut_with_nested_object() {
    use serde_json::{json, Value, Map};

    let mut v = json!({ "a": { "b": { "c": true } } });

    // Should be able to mutate the nested object
    let map = v["a"].as_object_mut().unwrap();
    map.insert("d".to_string(), json!(100));

    assert_eq!(v, json!({ "a": { "b": { "c": true }, "d": 100 } }));
}

#[test]
fn test_as_object_mut_with_empty_object() {
    use serde_json::{json, Value};

    let mut v = json!({});

    // Should return a mutable reference to the empty map
    let map = v.as_object_mut().unwrap();
    map.insert("new_key".to_string(), json!("new_value"));

    assert_eq!(v, json!({"new_key": "new_value"}));
}

#[test]
fn test_as_object_mut_with_array_value() {
    use serde_json::{json, Value};

    let mut v = json!([1, 2, 3]);

    // Should return None since self is not an object
    assert!(v.as_object_mut().is_none());
}

#[test]
fn test_as_object_mut_with_string_value() {
    use serde_json::{json, Value};

    let mut v = json!("not_an_object");

    // Should return None since self is not an object
    assert!(v.as_object_mut().is_none());
}

#[test]
fn test_as_object_mut_with_bool_value() {
    use serde_json::{json, Value};

    let mut v = json!(false);

    // Should return None since self is not an object
    assert!(v.as_object_mut().is_none());
}

