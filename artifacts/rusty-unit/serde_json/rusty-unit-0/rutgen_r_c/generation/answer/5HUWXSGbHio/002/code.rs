// Answer 0

#[test]
fn test_display_null() {
    let value = Value::Null;
    let value_type = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", value_type);
    assert!(result.is_ok());
    assert_eq!(buffer, "null");
}

#[test]
fn test_display_bool() {
    let value = Value::Bool(true);
    let value_type = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", value_type);
    assert!(result.is_ok());
    assert_eq!(buffer, "boolean");
}

#[test]
fn test_display_number() {
    let value = Value::Number(Number::from(13.3)); // Assuming a suitable Number implementation available
    let value_type = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", value_type);
    assert!(result.is_ok());
    assert_eq!(buffer, "number");
}

#[test]
fn test_display_string() {
    let value = Value::String(String::from("test string"));
    let value_type = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", value_type);
    assert!(result.is_ok());
    assert_eq!(buffer, "string");
}

#[test]
fn test_display_array() {
    let value = Value::Array(vec![Value::Null, Value::Bool(false)]);
    let value_type = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", value_type);
    assert!(result.is_ok());
    assert_eq!(buffer, "array");
}

#[test]
fn test_display_object() {
    let mut object_map = Map::new(); // Assuming a suitable Map implementation available
    object_map.insert(String::from("key"), Value::Bool(true));
    let value = Value::Object(object_map);
    let value_type = Type(&value);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", value_type);
    assert!(result.is_ok());
    assert_eq!(buffer, "object");
}

