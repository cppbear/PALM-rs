// Answer 0

#[test]
fn test_is_boolean_with_true_value() {
    use serde_json::json;
    
    let v = json!(true);
    assert!(v.is_boolean());
}

#[test]
fn test_is_boolean_with_false_value() {
    use serde_json::json;
    
    let v = json!(false);
    assert!(v.is_boolean());
}

#[test]
fn test_is_boolean_with_integer_value() {
    use serde_json::json;
    
    let v = json!(42);
    assert!(!v.is_boolean());
}

#[test]
fn test_is_boolean_with_string_value() {
    use serde_json::json;
    
    let v = json!("true");
    assert!(!v.is_boolean());
}

#[test]
fn test_is_boolean_with_array_value() {
    use serde_json::json;
    
    let v = json!([true, false]);
    assert!(!v.is_boolean());
}

#[test]
fn test_is_boolean_with_object_value() {
    use serde_json::json;
    
    let v = json!({"key": "value"});
    assert!(!v.is_boolean());
}

#[test]
fn test_is_boolean_with_null_value() {
    use serde_json::json;
    
    let v = json!(null);
    assert!(!v.is_boolean());
}

