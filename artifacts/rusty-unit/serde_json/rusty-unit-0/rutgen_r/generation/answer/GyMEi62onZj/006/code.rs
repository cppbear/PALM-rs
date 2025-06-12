// Answer 0

#[test]
fn test_fmt_value_null() {
    struct Value;

    impl Value {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Value::Null => formatter.write_str("Null"),
                Value::Bool(_) => write!(formatter, "Bool(true)"), // Placeholder for struct
                Value::Number(_) => unreachable!(), // Placeholder for struct
                Value::String(_) => unreachable!(), // Placeholder for struct
                Value::Array(_) => unreachable!(), // Placeholder for struct
                Value::Object(_) => unreachable!(), // Placeholder for struct
            }
        }

        fn null() -> Self { Value::Null }
    }

    let value = Value::null();
    let mut buffer = String::new();
    let result = value.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert_eq!(buffer, "Null");
    assert!(result.is_ok());
}

#[test]
fn test_fmt_value_bool() {
    struct Value {
        kind: Kind,
    }

    enum Kind {
        Bool(bool),
    }

    impl Value {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match &self.kind {
                Kind::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            }
        }

        fn bool(value: bool) -> Self {
            Value { kind: Kind::Bool(value) }
        }
    }

    let value = Value::bool(true);
    let mut buffer = String::new();
    let result = value.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert_eq!(buffer, "Bool(true)");
    assert!(result.is_ok());
}

#[test]
fn test_fmt_value_number() {
    struct Value {
        kind: Kind,
    }

    enum Kind {
        Number(u32),
    }

    impl Value {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match &self.kind {
                Kind::Number(number) => write!(formatter, "Number({})", number),
            }
        }

        fn number(value: u32) -> Self {
            Value { kind: Kind::Number(value) }
        }
    }

    let value = Value::number(42);
    let mut buffer = String::new();
    let result = value.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert_eq!(buffer, "Number(42)");
    assert!(result.is_ok());

    let value_zero = Value::number(0);
    let mut buffer_zero = String::new();
    let result_zero = value_zero.fmt(&mut std::fmt::Formatter::new(&mut buffer_zero));
    assert_eq!(buffer_zero, "Number(0)");
    assert!(result_zero.is_ok());
}

#[test]
fn test_fmt_value_string() {
    struct Value {
        kind: Kind,
    }

    enum Kind {
        String(String),
    }

    impl Value {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match &self.kind {
                Kind::String(string) => write!(formatter, "String({:?})", string),
            }
        }

        fn string(value: &str) -> Self {
            Value { kind: Kind::String(value.to_string()) }
        }
    }

    let value = Value::string("test");
    let mut buffer = String::new();
    let result = value.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert_eq!(buffer, r#"String("test")"#);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_value_array() {
    struct Value {
        kind: Kind,
    }

    enum Kind {
        Array(Vec<u32>),
    }

    impl Value {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match &self.kind {
                Kind::Array(vec) => {
                    formatter.write_str("Array ")?;
                    write!(formatter, "{:?}", vec)
                }
            }
        }

        fn array(value: Vec<u32>) -> Self {
            Value { kind: Kind::Array(value) }
        }
    }

    let value = Value::array(vec![1, 2, 3]);
    let mut buffer = String::new();
    let result = value.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert_eq!(buffer, "Array [1, 2, 3]");
    assert!(result.is_ok());
}

#[test]
fn test_fmt_value_object() {
    struct Value {
        kind: Kind,
    }

    enum Kind {
        Object(std::collections::HashMap<String, u32>),
    }

    impl Value {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match &self.kind {
                Kind::Object(map) => {
                    formatter.write_str("Object ")?;
                    write!(formatter, "{:?}", map)
                }
            }
        }

        fn object(value: std::collections::HashMap<String, u32>) -> Self {
            Value { kind: Kind::Object(value) }
        }
    }

    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), 1);
    let value = Value::object(map);
    let mut buffer = String::new();
    let result = value.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert_eq!(buffer, "Object {\"key1\": 1}");
    assert!(result.is_ok());
}

