// Answer 0

#[test]
fn test_display_null() {
    let value = Value::Null;
    let display_value = Type(&value);
    let mut output = String::new();
    let result = write!(&mut output, "{}", display_value);
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_display_bool() {
    let value = Value::Bool(true);
    let display_value = Type(&value);
    let mut output = String::new();
    let result = write!(&mut output, "{}", display_value);
    assert!(result.is_ok());
    assert_eq!(output, "boolean");
}

#[test]
fn test_display_number() {
    let value = Value::Number(Number::from(12.5));
    let display_value = Type(&value);
    let mut output = String::new();
    let result = write!(&mut output, "{}", display_value);
    assert!(result.is_ok());
    assert_eq!(output, "number");
}

#[test]
fn test_display_string() {
    let value = Value::String(String::from("a string"));
    let display_value = Type(&value);
    let mut output = String::new();
    let result = write!(&mut output, "{}", display_value);
    assert!(result.is_ok());
    assert_eq!(output, "string");
}

#[test]
fn test_display_array() {
    let value = Value::Array(vec![Value::Null, Value::Bool(true)]);
    let display_value = Type(&value);
    let mut output = String::new();
    let result = write!(&mut output, "{}", display_value);
    assert!(result.is_ok());
    assert_eq!(output, "array");
}

#[test]
fn test_display_object() {
    let mut obj = Map::new();
    obj.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(obj);
    let display_value = Type(&value);
    let mut output = String::new();
    let result = write!(&mut output, "{}", display_value);
    assert!(result.is_ok());
    assert_eq!(output, "object");
}

