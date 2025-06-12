// Answer 0

#[test]
fn test_as_str_with_string_value() {
    let value = Value::String(String::from("some string"));
    assert_eq!(value.as_str(), Some("some string"));
}

#[test]
fn test_as_str_with_non_string_value() {
    let value = Value::Bool(false);
    assert_eq!(value.as_str(), None);
}

#[test]
fn test_as_str_with_null_value() {
    let value = Value::Null;
    assert_eq!(value.as_str(), None);
}

#[test]
fn test_as_str_with_empty_string() {
    let value = Value::String(String::from(""));
    assert_eq!(value.as_str(), Some(""));
}

