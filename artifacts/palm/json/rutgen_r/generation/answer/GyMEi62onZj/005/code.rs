// Answer 0

#[test]
fn test_fmt_string() {
    use std::fmt::{self, Debug};
    
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }

    impl Value {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Value::Null => formatter.write_str("Null"),
                Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
                Value::Number(number) => Debug::fmt(number, formatter),
                Value::String(string) => write!(formatter, "String({:?})", string),
                Value::Array(vec) => {
                    formatter.write_str("Array ")?;
                    Debug::fmt(vec, formatter)
                }
                Value::Object(map) => {
                    formatter.write_str("Object ")?;
                    Debug::fmt(map, formatter)
                }
            }
        }
    }

    let value = Value::String(String::from("test_string"));
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    
    assert!(result.is_ok());
    assert_eq!(output, "String(\"test_string\")");
}

