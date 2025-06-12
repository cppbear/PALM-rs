// Answer 0

#[test]
fn test_is_object_with_object() {
    use serde_json::json;
    let obj = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    
    assert!(obj.is_object());
    assert!(obj["a"].is_object());
}

#[test]
fn test_is_object_with_array() {
    use serde_json::json;
    let obj = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    
    assert!(!obj["b"].is_object());
}

#[test]
fn test_is_object_with_empty_object() {
    use serde_json::json;
    let obj = json!({});
    
    assert!(obj.is_object());
}

#[test]
fn test_is_object_with_string() {
    use serde_json::json;
    let obj = json!("a string");
    
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_with_number() {
    use serde_json::json;
    let obj = json!(42);
    
    assert!(!obj.is_object());
}

#[test]
fn test_is_object_with_null() {
    use serde_json::json;
    let obj = json!(null);
    
    assert!(!obj.is_object());
}

