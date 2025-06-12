// Answer 0

#[test]
fn test_eq_u64_value_is_u64() {
    let value = Value::Number(Number::from(42u64));
    let result = eq_u64(&value, 42);
    assert!(result);
}

#[test]
fn test_eq_u64_value_is_u64_different() {
    let value = Value::Number(Number::from(42u64));
    let result = eq_u64(&value, 100);
    assert!(!result);
}

#[test]
fn test_eq_u64_value_is_not_number() {
    let value = Value::Bool(true);
    let result = eq_u64(&value, 42);
    assert!(!result);
}

#[test]
fn test_eq_u64_value_is_null() {
    let value = Value::Null;
    let result = eq_u64(&value, 42);
    assert!(!result);
}

#[test]
fn test_eq_u64_value_is_string() {
    let value = Value::String(String::from("42"));
    let result = eq_u64(&value, 42);
    assert!(!result);
}

