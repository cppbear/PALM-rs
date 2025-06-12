// Answer 0

#[test]
fn test_is_i64_with_non_number_value() {
    use serde_json::json;
    
    let v = json!({"key": "value"}); // A JSON object, not a number.
    
    assert!(!v["key"].is_i64()); // should return false because it's not a number.
}

#[test]
fn test_is_i64_with_null_value() {
    use serde_json::json;
    
    let v = json!(null); // A JSON null value.
    
    assert!(!v.is_i64()); // should return false, as null is not a number.
}

#[test]
fn test_is_i64_with_boolean_value() {
    use serde_json::json;
    
    let v_true = json!(true); // A JSON boolean true.
    let v_false = json!(false); // A JSON boolean false.
    
    assert!(!v_true.is_i64()); // should return false, as true is not a number.
    assert!(!v_false.is_i64()); // should return false, as false is not a number.
}

#[test]
fn test_is_i64_with_array_value() {
    use serde_json::json;
    
    let v = json!([1, 2, 3]); // A JSON array.
    
    assert!(!v.is_i64()); // should return false, as an array is not a number.
}

#[test]
fn test_is_i64_with_nested_object() {
    use serde_json::json;
    
    let v = json!({"nested": {"key": "value"}}); // A nested JSON object.
    
    assert!(!v["nested"].is_i64()); // should return false, it's not a number.
}

