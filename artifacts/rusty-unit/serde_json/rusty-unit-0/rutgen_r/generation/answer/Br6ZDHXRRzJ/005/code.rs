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

#[derive(Debug, PartialEq)]
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
            Value::Number(n) => n.unexpected(),
            Value::String(s) => Unexpected::Str(s.clone()),
            Value::Array(_) => Unexpected::Seq,
            Value::Object(_) => Unexpected::Map,
        }
    }
}

#[test]
fn test_unexpected_bool_true() {
    let value = Value::Bool(true);
    let result = value.unexpected();
    assert_eq!(result, Unexpected::Bool(true));
}

#[test]
fn test_unexpected_bool_false() {
    let value = Value::Bool(false);
    let result = value.unexpected();
    assert_eq!(result, Unexpected::Bool(false));
}

