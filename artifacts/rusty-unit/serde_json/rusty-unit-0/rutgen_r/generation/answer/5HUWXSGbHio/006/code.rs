// Answer 0

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

struct MyValue(Value);

impl std::fmt::Display for MyValue {
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
    let value = MyValue(Value::Null);
    let mut buffer = String::new();
    assert_eq!(value.fmt(&mut std::fmt::Formatter::new(&mut buffer)), Ok(()));
    assert_eq!(buffer, "null");
}

#[test]
fn test_fmt_bool() {
    let value = MyValue(Value::Bool(true));
    let mut buffer = String::new();
    assert_eq!(value.fmt(&mut std::fmt::Formatter::new(&mut buffer)), Ok(()));
    assert_eq!(buffer, "boolean");
}

#[test]
fn test_fmt_number() {
    let value = MyValue(Value::Number(42));
    let mut buffer = String::new();
    assert_eq!(value.fmt(&mut std::fmt::Formatter::new(&mut buffer)), Ok(()));
    assert_eq!(buffer, "number");
}

#[test]
fn test_fmt_string() {
    let value = MyValue(Value::String("test".to_string()));
    let mut buffer = String::new();
    assert_eq!(value.fmt(&mut std::fmt::Formatter::new(&mut buffer)), Ok(()));
    assert_eq!(buffer, "string");
}

#[test]
fn test_fmt_array() {
    let value = MyValue(Value::Array(vec![Value::Null]));
    let mut buffer = String::new();
    assert_eq!(value.fmt(&mut std::fmt::Formatter::new(&mut buffer)), Ok(()));
    assert_eq!(buffer, "array");
}

#[test]
fn test_fmt_object() {
    let mut map = std::collections::HashMap::new();
    map.insert("key".to_string(), Value::Null);
    let value = MyValue(Value::Object(map));
    let mut buffer = String::new();
    assert_eq!(value.fmt(&mut std::fmt::Formatter::new(&mut buffer)), Ok(()));
    assert_eq!(buffer, "object");
}

