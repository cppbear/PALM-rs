// Answer 0

#[test]
fn test_fmt_array_panic() {
    struct MockFormatter {
        buffer: String,
        should_panic: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_panic {
                return Err(fmt::Error);
            }
            self.buffer.push_str(s);
            Ok(())
        }
    }

    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }

    #[allow(dead_code)]
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

    let mut mock_formatter = MockFormatter {
        buffer: String::new(),
        should_panic: true,
    };

    let value = Value::Array(vec![]);

    let result = value.fmt(&mut mock_formatter);
    
    assert!(result.is_err());
}

