// Answer 0

#[test]
fn test_eq_str_with_string_value() {
    let value = Value::String("test".to_string());
    assert_eq!(eq_str(&value, "test"), true);
}

#[test]
fn test_eq_str_with_matching_string_value() {
    let value = Value::String("hello".to_string());
    assert_eq!(eq_str(&value, "hello"), true);
}

#[test]
fn test_eq_str_with_non_matching_string_value() {
    let value = Value::String("world".to_string());
    assert_eq!(eq_str(&value, "hello"), false);
}

#[test]
fn test_eq_str_with_null_value() {
    let value = Value::Null;
    assert_eq!(eq_str(&value, "null"), false);
}

#[test]
fn test_eq_str_with_boolean_value() {
    let value = Value::Bool(true);
    assert_eq!(eq_str(&value, "true"), false);
}

#[test]
fn test_eq_str_with_number_value() {
    let value = Value::Number(Number::from(42)); // Assuming a `Number` representation exists
    assert_eq!(eq_str(&value, "42"), false);
}

#[test]
fn test_eq_str_with_empty_string() {
    let value = Value::String("".to_string());
    assert_eq!(eq_str(&value, ""), true);
}

#[test]
fn test_eq_str_with_non_string_value() {
    let value = Value::Array(vec![]);
    assert_eq!(eq_str(&value, "non-string"), false);
}

#[test]
fn test_eq_str_with_string_and_none() {
    let value = Value::String("some string".to_string());
    assert_eq!(eq_str(&value, "none"), false);
}

