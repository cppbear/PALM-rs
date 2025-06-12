// Answer 0

#[test]
fn test_as_object_mut_with_object() {
    use serde_json::{json, Value, Map};
    
    let mut v = json!({ "a": { "nested": true } });
    
    let map = v.as_object_mut().unwrap();
    map.clear();
    
    assert_eq!(v, json!({ "a": {} }));
}

#[test]
fn test_as_object_mut_with_non_object() {
    use serde_json::json;
    
    let mut v = json!(42);
    
    assert!(v.as_object_mut().is_none());
}

#[test]
fn test_as_object_mut_with_empty_object() {
    use serde_json::{json, Value, Map};
    
    let mut v = json!({ "a": {} });
    
    let map = v.as_object_mut().unwrap();
    map.insert("b".to_string(), json!(true));
    
    assert_eq!(v, json!({ "a": { "b": true } }));
}

