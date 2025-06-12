// Answer 0

#[test]
fn test_display_value_null() {
    use super::Value;

    let value = Value::Null;
    let type_value = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", type_value);
    assert!(result.is_ok());
    assert_eq!(buffer, "null");
}

#[test]
fn test_display_value_bool() {
    use super::Value;

    let value = Value::Bool(true);
    let type_value = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", type_value);
    assert!(result.is_ok());
    assert_eq!(buffer, "boolean");
}

#[test]
fn test_display_value_number() {
    use super::Value;

    let value = Value::Number(Number::from(1.0));
    let type_value = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", type_value);
    assert!(result.is_ok());
    assert_eq!(buffer, "number");
}

#[test]
fn test_display_value_string() {
    use super::Value;

    let value = Value::String("test".to_owned());
    let type_value = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", type_value);
    assert!(result.is_ok());
    assert_eq!(buffer, "string");
}

#[test]
fn test_display_value_array() {
    use super::Value;

    let value = Value::Array(vec![Value::String("item".to_owned())]);
    let type_value = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", type_value);
    assert!(result.is_ok());
    assert_eq!(buffer, "array");
}

#[test]
fn test_display_value_object() {
    use super::Value;
    use alloc::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert("key".to_owned(), Value::String("value".to_owned()));
    let value = Value::Object(map);
    let type_value = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", type_value);
    assert!(result.is_ok());
    assert_eq!(buffer, "object");
}

