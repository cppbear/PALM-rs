// Answer 0

#[test]
fn test_fmt_null() {
    struct TestValue(Value);
    impl fmt::Display for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value = TestValue(Value::Null);
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_fmt_boolean() {
    struct TestValue(Value);
    impl fmt::Display for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value_true = TestValue(Value::Bool(true));
    let value_false = TestValue(Value::Bool(false));
    
    let mut output_true = String::new();
    let mut output_false = String::new();
    
    let result_true = write!(&mut output_true, "{}", value_true);
    let result_false = write!(&mut output_false, "{}", value_false);
    
    assert!(result_true.is_ok());
    assert!(result_false.is_ok());
    assert_eq!(output_true, "boolean");
    assert_eq!(output_false, "boolean");
}

#[test]
fn test_fmt_number() {
    struct TestValue(Value);
    impl fmt::Display for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value = TestValue(Value::Number(42.into()));
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "number");
}

#[test]
fn test_fmt_string() {
    struct TestValue(Value);
    impl fmt::Display for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value = TestValue(Value::String("example".into()));
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "string");
}

#[test]
fn test_fmt_array() {
    struct TestValue(Value);
    impl fmt::Display for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value = TestValue(Value::Array(vec![]));
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "array");
}

#[test]
fn test_fmt_object() {
    struct TestValue(Value);
    impl fmt::Display for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => formatter.write_str("null"),
                Value::Bool(_) => formatter.write_str("boolean"),
                Value::Number(_) => formatter.write_str("number"),
                Value::String(_) => formatter.write_str("string"),
                Value::Array(_) => formatter.write_str("array"),
                Value::Object(_) => formatter.write_str("object"),
            }
        }
    }

    let value = TestValue(Value::Object(serde_json::Map::new()));
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "object");
}

