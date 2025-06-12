// Answer 0

#[test]
fn test_is_f64_with_integer_value() {
    use serde_json::json;
    
    let v = json!(42);
    
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_unsigned_integer_value() {
    use serde_json::json;
    
    let v = json!(100u64);
    
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_negative_integer_value() {
    use serde_json::json;
    
    let v = json!(-10);
    
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_null_value() {
    use serde_json::json;
    
    let v = json!(null);
    
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_boolean_value() {
    use serde_json::json;
    
    let v_true = json!(true);
    let v_false = json!(false);
    
    assert!(!v_true.is_f64());
    assert!(!v_false.is_f64());
}

#[test]
fn test_is_f64_with_string_value() {
    use serde_json::json;
    
    let v = json!("not a number");
    
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_array_value() {
    use serde_json::json;
    
    let v = json!([1, 2, 3]);
    
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_object_value() {
    use serde_json::json;
    
    let v = json!({"key": "value"});
    
    assert!(!v.is_f64());
}

