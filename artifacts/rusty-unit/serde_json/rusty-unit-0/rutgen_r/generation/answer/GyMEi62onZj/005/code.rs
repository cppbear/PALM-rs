// Answer 0

#[test]
fn test_value_fmt_string() {
    use std::fmt;
    use std::fmt::Debug;

    enum Value {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }

    impl fmt::Display for Value {
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

    let value_string = Value::String("test".to_string());
    let result = format!("{}", value_string);
    assert_eq!(result, "String(\"test\")");

    let value_string_empty = Value::String(String::new());
    let result_empty = format!("{}", value_string_empty);
    assert_eq!(result_empty, "String(\"\")");
}

