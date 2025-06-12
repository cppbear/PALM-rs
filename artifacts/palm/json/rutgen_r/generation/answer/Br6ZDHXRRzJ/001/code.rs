// Answer 0

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(i32), // Simplified for the test
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>), // Key-value pairs for Object
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
            Value::Number(n) => Unexpected::Map, // Assuming a Number should call unexpected
            Value::String(s) => Unexpected::Str(s.clone()),
            Value::Array(_) => Unexpected::Seq,
            Value::Object(_) => Unexpected::Map,
        }
    }
}

#[test]
fn test_unexpected_with_value_object() {
    let test_obj = Value::Object(vec![
        ("key1".to_string(), Value::String("value1".to_string())),
        ("key2".to_string(), Value::Number(42)),
    ]);

    match test_obj.unexpected() {
        Unexpected::Map => (),
        _ => panic!("Expected Unexpected::Map"),
    }
}

