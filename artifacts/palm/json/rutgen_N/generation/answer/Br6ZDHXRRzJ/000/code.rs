// Answer 0

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

#[derive(Debug)]
enum Unexpected {
    Unit,
    Bool(bool),
    Str(String),
    Seq,
    Map,
}

impl Value {
    fn unexpected(&self) -> Unexpected {
        match self {
            Value::Null => Unexpected::Unit,
            Value::Bool(b) => Unexpected::Bool(*b),
            Value::Number(n) => Unexpected::Str(n.to_string()), // Assuming a conversion for the example
            Value::String(s) => Unexpected::Str(s.clone()),
            Value::Array(_) => Unexpected::Seq,
            Value::Object(_) => Unexpected::Map,
        }
    }
}

#[test]
fn test_unexpected_null() {
    let value = Value::Null;
    assert_eq!(value.unexpected(), Unexpected::Unit);
}

#[test]
fn test_unexpected_bool() {
    let value_true = Value::Bool(true);
    let value_false = Value::Bool(false);
    assert_eq!(value_true.unexpected(), Unexpected::Bool(true));
    assert_eq!(value_false.unexpected(), Unexpected::Bool(false));
}

#[test]
fn test_unexpected_number() {
    let value = Value::Number(42);
    assert_eq!(value.unexpected(), Unexpected::Str("42".to_string())); // Assuming to_str conversion
}

#[test]
fn test_unexpected_string() {
    let value = Value::String("hello".to_string());
    assert_eq!(value.unexpected(), Unexpected::Str("hello".to_string()));
}

#[test]
fn test_unexpected_array() {
    let value = Value::Array(vec![Value::Null, Value::Bool(true)]);
    assert_eq!(value.unexpected(), Unexpected::Seq);
}

#[test]
fn test_unexpected_object() {
    let value = Value::Object(std::collections::HashMap::new());
    assert_eq!(value.unexpected(), Unexpected::Map);
}

