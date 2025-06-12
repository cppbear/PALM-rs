// Answer 0

#[test]
fn test_display_null() {
    let value = Value::Null;
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    let result = type_instance.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.to_string(), "null");
}

#[test]
fn test_display_bool_true() {
    let value = Value::Bool(true);
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    let result = type_instance.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.to_string(), "boolean");
}

#[test]
fn test_display_bool_false() {
    let value = Value::Bool(false);
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    let result = type_instance.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.to_string(), "boolean");
}

#[test]
fn test_display_number() {
    let value = Value::Number(Number::from(12.5));
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    let result = type_instance.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.to_string(), "number");
}

#[test]
fn test_display_string() {
    let value = Value::String("a string".to_owned());
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    let result = type_instance.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.to_string(), "string");
}

#[test]
fn test_display_array() {
    let value = Value::Array(vec![Value::String("an".to_owned()), Value::String("array".to_owned())]);
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    let result = type_instance.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.to_string(), "array");
}

#[test]
fn test_display_object() {
    let mut map = Map::new();
    map.insert("key".to_owned(), Value::String("value".to_owned()));
    let value = Value::Object(map);
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    let result = type_instance.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.to_string(), "object");
}

