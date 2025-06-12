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

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => std::fmt::Debug::fmt(number, formatter),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(vec) => {
                formatter.write_str("Array ")?;
                std::fmt::Debug::fmt(vec, formatter)
            }
            Value::Object(map) => {
                formatter.write_str("Object ")?;
                std::fmt::Debug::fmt(map, formatter)
            }
        }
    }
}

#[test]
fn test_value_object_panic() {
    let mut formatter = MyFormatter::new();
    
    let map = std::collections::HashMap::new();
    let value = Value::Object(map);
    
    let result = std::panic::catch_unwind(|| {
        let _ = value.fmt(&mut formatter);
    });

    assert!(result.is_err());
}

struct MyFormatter {
    // Custom structure to overwhelm
}

impl MyFormatter {
    fn new() -> Self {
        MyFormatter { /* Initialization Code */ }
    }
}

impl std::fmt::Write for MyFormatter {
    fn write_str(&mut self, _s: &str) -> std::fmt::Result {
        // Simulate an error while writing to formatter
        Err(std::fmt::Error)
    }
}

