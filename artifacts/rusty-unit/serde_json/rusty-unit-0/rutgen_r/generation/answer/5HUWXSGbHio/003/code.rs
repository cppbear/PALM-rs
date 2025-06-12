// Answer 0

#[test]
fn test_fmt_string_value() {
    use serde_json::Value;
    use std::fmt;

    struct TestValue(Value);

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::String(ref s) => formatter.write_str(&s),
                _ => formatter.write_str("not a string"),
            }
        }
    }

    let string_value = TestValue(Value::String("hello".to_string()));
    let mut output = String::new();
    {
        let mut formatter = formatter::Formatter::new(&mut output);
        let _ = string_value.fmt(&mut formatter);
    }
    assert_eq!(output, "hello");
}

#[test]
fn test_fmt_null_value() {
    use serde_json::Value;
    use std::fmt;

    struct TestValue(Value);

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                _ => formatter.write_str("not null"),
            }
        }
    }

    let null_value = TestValue(Value::Null);
    let mut output = String::new();
    {
        let mut formatter = formatter::Formatter::new(&mut output);
        let _ = null_value.fmt(&mut formatter);
    }
    assert_eq!(output, "null");
}

#[test]
fn test_fmt_bool_value() {
    use serde_json::Value;
    use std::fmt;

    struct TestValue(Value);

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Bool(_) => formatter.write_str("boolean"),
                _ => formatter.write_str("not a boolean"),
            }
        }
    }

    let bool_value = TestValue(Value::Bool(true));
    let mut output = String::new();
    {
        let mut formatter = formatter::Formatter::new(&mut output);
        let _ = bool_value.fmt(&mut formatter);
    }
    assert_eq!(output, "boolean");
}

#[test]
fn test_fmt_number_value() {
    use serde_json::Value;
    use std::fmt;

    struct TestValue(Value);

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Number(_) => formatter.write_str("number"),
                _ => formatter.write_str("not a number"),
            }
        }
    }

    let number_value = TestValue(Value::Number(serde_json::Number::from(42)));
    let mut output = String::new();
    {
        let mut formatter = formatter::Formatter::new(&mut output);
        let _ = number_value.fmt(&mut formatter);
    }
    assert_eq!(output, "number");
}

#[test]
fn test_fmt_array_value() {
    use serde_json::Value;
    use std::fmt;

    struct TestValue(Value);

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Array(_) => formatter.write_str("array"),
                _ => formatter.write_str("not an array"),
            }
        }
    }

    let array_value = TestValue(Value::Array(vec![Value::Number(serde_json::Number::from(1))]));
    let mut output = String::new();
    {
        let mut formatter = formatter::Formatter::new(&mut output);
        let _ = array_value.fmt(&mut formatter);
    }
    assert_eq!(output, "array");
}

#[test]
fn test_fmt_object_value() {
    use serde_json::Value;
    use std::fmt;

    struct TestValue(Value);

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Object(_) => formatter.write_str("object"),
                _ => formatter.write_str("not an object"),
            }
        }
    }

    let object_value = TestValue(Value::Object(serde_json::Map::new()));
    let mut output = String::new();
    {
        let mut formatter = formatter::Formatter::new(&mut output);
        let _ = object_value.fmt(&mut formatter);
    }
    assert_eq!(output, "object");
}

