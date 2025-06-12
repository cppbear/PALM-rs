// Answer 0

#[test]
fn test_eq_i64_with_null() {
    let value = Value::Null;
    assert!(!eq_i64(&value, 0));
}

#[test]
fn test_eq_i64_with_boolean() {
    let value = Value::Bool(true);
    assert!(!eq_i64(&value, 1));
}

#[test]
fn test_eq_i64_with_string() {
    let value = Value::String("test".to_string());
    assert!(!eq_i64(&value, 0));
}

#[test]
fn test_eq_i64_with_number() {
    let value = Value::Number(Number::from(42));
    assert!(eq_i64(&value, 42));
    assert!(!eq_i64(&value, 0));
}

#[test]
fn test_eq_i64_with_array() {
    let value = Value::Array(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]);
    assert!(!eq_i64(&value, 1));
}

#[test]
fn test_eq_i64_with_object() {
    let mut map = std::collections::BTreeMap::new();
    map.insert("key".to_string(), Value::Number(Number::from(10)));
    let value = Value::Object(map);
    assert!(!eq_i64(&value, 10));
}

#[test]
fn test_eq_i64_with_matching_value() {
    let value = Value::Number(Number::from(100));
    assert!(eq_i64(&value, 100));
} 

#[test]
fn test_eq_i64_with_non_matching_value() {
    let value = Value::Number(Number::from(57));
    assert!(!eq_i64(&value, 100));
}

