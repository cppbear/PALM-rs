// Answer 0

#[test]
fn test_is_boolean_true() {
    use serde_json::Value;
    
    let v_true = Value::Bool(true);
    let v_false = Value::Bool(false);
    
    assert!(v_true.is_boolean());
    assert!(v_false.is_boolean());
}

#[test]
fn test_is_boolean_false() {
    use serde_json::Value;
    
    let v_null = Value::Null;
    let v_string = Value::String("false".to_string());
    let v_number = Value::Number(Number { n: 0.0 });

    assert!(!v_null.is_boolean());
    assert!(!v_string.is_boolean());
    assert!(!v_number.is_boolean());
}

#[test]
fn test_is_boolean_with_object() {
    use serde_json::Value;

    let v_object = Value::Object(Map { map: Default::default() });

    assert!(!v_object.is_boolean());
}

#[test]
fn test_is_boolean_with_array() {
    use serde_json::Value;

    let v_array = Value::Array(vec![]);

    assert!(!v_array.is_boolean());
}

