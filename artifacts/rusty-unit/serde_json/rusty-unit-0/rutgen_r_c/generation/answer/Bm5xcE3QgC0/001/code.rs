// Answer 0

#[test]
fn test_eq_bool_true() {
    let value = Value::Bool(true);
    let result = eq_bool(&value, true);
    assert!(result);
}

#[test]
fn test_eq_bool_false() {
    let value = Value::Bool(false);
    let result = eq_bool(&value, false);
    assert!(result);
}

#[test]
fn test_eq_bool_mismatch_true_false() {
    let value = Value::Bool(false);
    let result = eq_bool(&value, true);
    assert!(!result);
}

#[test]
fn test_eq_bool_mismatch_false_true() {
    let value = Value::Bool(true);
    let result = eq_bool(&value, false);
    assert!(!result);
}

#[test]
fn test_eq_bool_null() {
    let value = Value::Null;
    let result = eq_bool(&value, true);
    assert!(!result);
}

#[test]
fn test_eq_bool_number() {
    let value = Value::Number(Number::from(1));
    let result = eq_bool(&value, true);
    assert!(!result);
}

#[test]
fn test_eq_bool_string() {
    let value = Value::String("test".to_string());
    let result = eq_bool(&value, true);
    assert!(!result);
}

#[test]
fn test_eq_bool_array() {
    let value = Value::Array(vec![]);
    let result = eq_bool(&value, true);
    assert!(!result);
}

#[test]
fn test_eq_bool_object() {
    let value = Value::Object(Map::new());
    let result = eq_bool(&value, true);
    assert!(!result);
}

