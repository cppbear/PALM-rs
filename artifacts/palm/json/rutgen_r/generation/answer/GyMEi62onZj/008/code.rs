// Answer 0

#[test]
fn test_fmt_null() {
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

    let value = Value::Null;
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "Null");
}

#[test]
fn test_fmt_bool() {
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

    let value_true = Value::Bool(true);
    let value_false = Value::Bool(false);
    let mut output_true = String::new();
    let result_true = value_true.fmt(&mut fmt::Formatter::new(&mut output_true));
    
    let mut output_false = String::new();
    let result_false = value_false.fmt(&mut fmt::Formatter::new(&mut output_false));

    assert!(result_true.is_ok());
    assert_eq!(output_true, "Bool(true)");

    assert!(result_false.is_ok());
    assert_eq!(output_false, "Bool(false)");
}

#[test]
fn test_fmt_number() {
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

    let value = Value::Number(42);
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "42");
}

