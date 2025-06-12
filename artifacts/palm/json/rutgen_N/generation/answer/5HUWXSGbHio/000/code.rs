// Answer 0

#[derive(Debug)]
struct CustomValue(Value);

impl std::fmt::Debug for CustomValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self.0 {
            Value::Null => formatter.write_str("null"),
            Value::Bool(_) => formatter.write_str("boolean"),
            Value::Number(_) => formatter.write_str("number"),
            Value::String(_) => formatter.write_str("string"),
            Value::Array(_) => formatter.write_str("array"),
            Value::Object(_) => formatter.write_str("object"),
        }
    }
}

#[test]
fn test_fmt_null() {
    let value = CustomValue(Value::Null);
    let mut buffer = String::new();
    let result = value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "null");
}

#[test]
fn test_fmt_bool() {
    let value = CustomValue(Value::Bool(true));
    let mut buffer = String::new();
    let result = value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "boolean");
}

#[test]
fn test_fmt_number() {
    let value = CustomValue(Value::Number(serde_json::Number::from(42)));
    let mut buffer = String::new();
    let result = value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "number");
}

#[test]
fn test_fmt_string() {
    let value = CustomValue(Value::String("example".to_string()));
    let mut buffer = String::new();
    let result = value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "string");
}

#[test]
fn test_fmt_array() {
    let value = CustomValue(Value::Array(vec![]));
    let mut buffer = String::new();
    let result = value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "array");
}

#[test]
fn test_fmt_object() {
    let value = CustomValue(Value::Object(serde_json::Map::new()));
    let mut buffer = String::new();
    let result = value.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "object");
}

