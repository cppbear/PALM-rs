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

impl std::fmt::Debug for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => write!(formatter, "Number({})", number),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(vec) => {
                formatter.write_str("Array ")?;
                write!(formatter, "{:?}", vec)
            }
            Value::Object(map) => {
                formatter.write_str("Object ")?;
                write!(formatter, "{:?}", map)
            }
        }
    }
}

#[test]
fn test_value_fmt_null() {
    let value = Value::Null;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Null");
}

#[test]
fn test_value_fmt_bool() {
    let value = Value::Bool(true);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Bool(true)");
}

#[test]
fn test_value_fmt_number() {
    let value = Value::Number(42);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Number(42)");
}

#[test]
fn test_value_fmt_string() {
    let value = Value::String("Hello".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    assert!(result.is_ok());
    assert_eq!(output, "String(\"Hello\")");
}

#[test]
fn test_value_fmt_array() {
    let value = Value::Array(vec![Value::Number(1), Value::Bool(false)]);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Array [Number(1), Bool(false)]");
}

#[test]
fn test_value_fmt_object() {
    let mut map = std::collections::HashMap::new();
    map.insert("key".to_string(), Value::String("value".to_string()));
    let value = Value::Object(map);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Object {\"key\": String(\"value\")}");   
}

