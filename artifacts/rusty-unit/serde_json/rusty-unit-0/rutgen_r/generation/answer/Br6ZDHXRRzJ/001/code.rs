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
            Value::Number(n) => Unexpected::Seq, // For simplicity, assume Number returns Seq in unexpected
            Value::String(s) => Unexpected::Str(s.clone()),
            Value::Array(_) => Unexpected::Seq,
            Value::Object(_) => Unexpected::Map,
        }
    }
}

#[test]
fn test_unexpected_for_value_object() {
    let mut object = std::collections::HashMap::new();
    object.insert("key1".to_string(), Value::String("value1".to_string()));
    object.insert("key2".to_string(), Value::Number(42));

    let value = Value::Object(object);
    let result = value.unexpected();
    
    assert!(matches!(result, Unexpected::Map));
}

