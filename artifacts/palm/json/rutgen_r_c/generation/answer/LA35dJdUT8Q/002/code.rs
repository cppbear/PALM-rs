// Answer 0

#[test]
fn test_as_object_with_valid_object() {
    use serde_json::json;
    let v = json!({ "key": "value", "another_key": 42 });
    
    assert!(v.as_object().is_some());
    let map = v.as_object().unwrap();
    assert_eq!(map.len(), 2);
}

#[test]
fn test_as_object_with_empty_object() {
    use serde_json::json;
    let v = json!({});
    
    assert!(v.as_object().is_some());
    let map = v.as_object().unwrap();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_as_object_with_non_object() {
    use serde_json::json;
    let v = json!(true);

    assert!(v.as_object().is_none());
}

#[test]
fn test_as_object_with_nested_object() {
    use serde_json::json;
    let v = json!({ "outer": { "inner": "value" } });

    assert!(v.as_object().is_some());
    let map = v.as_object().unwrap();
    assert_eq!(map.len(), 1);
    assert!(map.get("outer").unwrap().as_object().is_some());
}

