// Answer 0

#[test]
fn test_fmt_null() {
    use core::fmt::Formatter;

    struct TestFormatter {
        output: String,
    }

    impl Formatter for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let value = Value::Null;
    let mut formatter = TestFormatter { output: String::new() };
    let result = value.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "Null");
}

#[test]
fn test_fmt_bool() {
    use core::fmt::Formatter;

    struct TestFormatter {
        output: String,
    }

    impl Formatter for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let value = Value::Bool(true);
    let mut formatter = TestFormatter { output: String::new() };
    let result = value.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "Bool(true)");
}

#[test]
fn test_fmt_number() {
    use core::fmt::Formatter;

    struct TestFormatter {
        output: String,
    }

    impl Formatter for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let number_value = Number { n: 12.5 };
    let value = Value::Number(number_value);
    let mut formatter = TestFormatter { output: String::new() };
    let result = value.fmt(&mut formatter);

    assert!(result.is_ok());
    // As we can't derive Debug for Number without implementation context, adjust expected output accordingly
    assert_eq!(formatter.output, "Number(12.5)"); // Adjust the expected string as necessary to match debug output of Number
}

#[test]
fn test_fmt_string() {
    use core::fmt::Formatter;

    struct TestFormatter {
        output: String,
    }

    impl Formatter for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let value = Value::String("a string".to_string());
    let mut formatter = TestFormatter { output: String::new() };
    let result = value.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, r#"String("a string")"#);
}

#[test]
fn test_fmt_array() {
    use core::fmt::Formatter;

    struct TestFormatter {
        output: String,
    }

    impl Formatter for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let array_value = Value::Array(vec![Value::String("item".to_string())]);
    let mut formatter = TestFormatter { output: String::new() };
    let result = array_value.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "Array String(\"item\")");
}

#[test]
fn test_fmt_object() {
    use core::fmt::Formatter;

    struct TestFormatter {
        output: String,
    }

    impl Formatter for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut object_map = Map::new();
    object_map.insert("key".to_string(), Value::String("value".to_string()));
    let object_value = Value::Object(object_map);
    let mut formatter = TestFormatter { output: String::new() };
    let result = object_value.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "Object Map {\"key\": String(\"value\")}");
}

