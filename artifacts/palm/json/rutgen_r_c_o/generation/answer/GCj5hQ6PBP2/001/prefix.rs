// Answer 0

#[test]
fn test_eq_str_string_equal() {
    let value = Value::String("test".to_string());
    let result = eq_str(&value, "test");
}

#[test]
fn test_eq_str_string_not_equal_different() {
    let value = Value::String("another test".to_string());
    let result = eq_str(&value, "test");
}

#[test]
fn test_eq_str_string_empty() {
    let value = Value::String("".to_string());
    let result = eq_str(&value, "");
}

#[test]
fn test_eq_str_string_null_as_str() {
    let value = Value::String("null".to_string());
    let result = eq_str(&value, "null");
}

#[test]
fn test_eq_str_null() {
    let value = Value::Null;
    let result = eq_str(&value, "test");
}

#[test]
fn test_eq_str_bool_true() {
    let value = Value::Bool(true);
    let result = eq_str(&value, "test");
}

#[test]
fn test_eq_str_bool_false() {
    let value = Value::Bool(false);
    let result = eq_str(&value, "test");
}

#[test]
fn test_eq_str_number() {
    let value = Value::Number(Number::from(0));
    let result = eq_str(&value, "test");
}

