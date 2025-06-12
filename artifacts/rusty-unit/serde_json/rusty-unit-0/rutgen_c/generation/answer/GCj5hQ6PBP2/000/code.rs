// Answer 0

#[test]
fn test_eq_str_with_string_value() {
    let value = Value::String("test".to_string());
    assert!(eq_str(&value, "test"));
    assert!(!eq_str(&value, "other"));
}

#[test]
fn test_eq_str_with_null_value() {
    let value = Value::Null;
    assert!(!eq_str(&value, "test"));
}

#[test]
fn test_eq_str_with_bool_value() {
    let value = Value::Bool(true);
    assert!(!eq_str(&value, "true"));
}

#[test]
fn test_eq_str_with_number_value() {
    let value = Value::Number(Number::from(42));
    assert!(!eq_str(&value, "42"));
}

#[test]
fn test_eq_str_with_empty_string() {
    let value = Value::String("".to_string());
    assert!(eq_str(&value, ""));
    assert!(!eq_str(&value, "non_empty"));
}

#[test]
fn test_eq_str_with_non_string_value() {
    let value = Value::Array(vec![]);
    assert!(!eq_str(&value, "test"));
}

