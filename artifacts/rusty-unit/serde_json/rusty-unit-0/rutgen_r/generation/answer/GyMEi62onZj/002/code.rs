// Answer 0

#[test]
fn test_value_fmt_object() {
    use std::fmt;
    use std::fmt::Debug;

    enum Value {
        Null,
        Bool(bool),
        Number(i32),
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

    // Test for Value::Object
    let mut map = std::collections::HashMap::new();
    map.insert(String::from("key1"), Value::Bool(true));
    map.insert(String::from("key2"), Value::Number(42));
    let value_object = Value::Object(map);

    let expected_output = "Object {\"key1\": Bool(true), \"key2\": 42}";
    let mut output = String::new();
    let result = write!(&mut output, "{}", value_object);

    assert!(result.is_ok());
    assert_eq!(output, expected_output);
}

#[test]
fn test_value_fmt_array() {
    use std::fmt;
    use std::fmt::Debug;

    enum Value {
        Null,
        Bool(bool),
        Number(i32),
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

    // Test for Value::Array
    let value_array = Value::Array(vec![Value::Bool(false), Value::Number(7)]);

    let expected_output = "Array [Bool(false), 7]";
    let mut output = String::new();
    let result = write!(&mut output, "{}", value_array);

    assert!(result.is_ok());
    assert_eq!(output, expected_output);
}

