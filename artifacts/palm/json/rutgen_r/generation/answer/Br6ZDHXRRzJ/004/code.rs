// Answer 0

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
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
            Value::Number(n) => Unexpected::Bool(*n > 0.0),  // Assuming a placeholder return for the number
            Value::String(s) => Unexpected::Str(s.clone()),
            Value::Array(_) => Unexpected::Seq,
            Value::Object(_) => Unexpected::Map,
        }
    }
}

#[test]
fn test_unexpected_with_number() {
    let positive_number = Value::Number(42.0);
    let negative_number = Value::Number(-1.0);
    let zero = Value::Number(0.0);
    
    // Test with a positive number
    match positive_number.unexpected() {
        Unexpected::Bool(true) => (),
        _ => panic!("Expected Unexpected::Bool(true) for positive number"),
    }

    // Test with a negative number
    match negative_number.unexpected() {
        Unexpected::Bool(false) => (),
        _ => panic!("Expected Unexpected::Bool(false) for negative number"),
    }

    // Test with zero
    match zero.unexpected() {
        Unexpected::Bool(false) => (),
        _ => panic!("Expected Unexpected::Bool(false) for zero"),
    }
}

