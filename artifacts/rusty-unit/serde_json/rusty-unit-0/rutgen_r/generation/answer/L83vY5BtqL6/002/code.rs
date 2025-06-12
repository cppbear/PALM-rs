// Answer 0

#[test]
fn test_as_object_mut_with_valid_object() {
    use serde_json::{json, Value, Map};

    let mut v = json!({ "a": { "nested": true } });

    if let Some(map) = v.as_object_mut() {
        map.clear();
        assert_eq!(v, json!({ "a": {} }));
    } else {
        panic!("Expected Value to be an Object");
    }
}

#[test]
fn test_as_object_mut_with_empty_object() {
    use serde_json::{json, Value, Map};

    let mut v = json!({});

    if let Some(map) = v.as_object_mut() {
        map.insert("key".to_string(), json!(42));
        assert_eq!(v, json!({"key": 42}));
    } else {
        panic!("Expected Value to be an Object");
    }
}

#[test]
fn test_as_object_mut_with_nested_object() {
    use serde_json::{json, Value, Map};

    let mut v = json!({ "outer": { "inner": { "value": true } } });

    if let Some(map) = v.as_object_mut() {
        if let Some(inner_map) = map.get_mut("outer").and_then(Value::as_object_mut) {
            inner_map.clear();
            assert_eq!(v, json!({"outer": { }}));
        } else {
            panic!("Expected nested Value to be an Object");
        }
    } else {
        panic!("Expected Value to be an Object");
    }
}

#[test]
fn test_as_object_mut_with_non_object() {
    use serde_json::{json, Value};

    let mut v = json!(42);

    assert!(v.as_object_mut().is_none(), "Expected None for non-object Value");
}

