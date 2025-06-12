// Answer 0

#[test]
fn test_fmt_value_null() {
    use std::fmt;
    use serde_json::Value;

    struct ValueWrapper(Value);

    impl fmt::Debug for ValueWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => f.write_str("null"),
                Value::Bool(_) => f.write_str("boolean"),
                Value::Number(_) => f.write_str("number"),
                Value::String(_) => f.write_str("string"),
                Value::Array(_) => f.write_str("array"),
                Value::Object(_) => f.write_str("object"),
            }
        }
    }

    let value = ValueWrapper(Value::Null);
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::from(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "null");
}

#[test]
fn test_fmt_value_bool() {
    use std::fmt;
    use serde_json::Value;

    struct ValueWrapper(Value);

    impl fmt::Debug for ValueWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => f.write_str("null"),
                Value::Bool(_) => f.write_str("boolean"),
                Value::Number(_) => f.write_str("number"),
                Value::String(_) => f.write_str("string"),
                Value::Array(_) => f.write_str("array"),
                Value::Object(_) => f.write_str("object"),
            }
        }
    }

    let value = ValueWrapper(Value::Bool(true));
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::from(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "boolean");
}

#[test]
fn test_fmt_value_number() {
    use std::fmt;
    use serde_json::Value;

    struct ValueWrapper(Value);

    impl fmt::Debug for ValueWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => f.write_str("null"),
                Value::Bool(_) => f.write_str("boolean"),
                Value::Number(_) => f.write_str("number"),
                Value::String(_) => f.write_str("string"),
                Value::Array(_) => f.write_str("array"),
                Value::Object(_) => f.write_str("object"),
            }
        }
    }

    let value = ValueWrapper(Value::Number(serde_json::Number::from(42)));
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::from(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "number");
}

#[test]
fn test_fmt_value_string() {
    use std::fmt;
    use serde_json::Value;

    struct ValueWrapper(Value);

    impl fmt::Debug for ValueWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => f.write_str("null"),
                Value::Bool(_) => f.write_str("boolean"),
                Value::Number(_) => f.write_str("number"),
                Value::String(_) => f.write_str("string"),
                Value::Array(_) => f.write_str("array"),
                Value::Object(_) => f.write_str("object"),
            }
        }
    }

    let value = ValueWrapper(Value::String("Hello".to_string()));
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::from(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "string");
}

#[test]
fn test_fmt_value_array() {
    use std::fmt;
    use serde_json::Value;

    struct ValueWrapper(Value);

    impl fmt::Debug for ValueWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => f.write_str("null"),
                Value::Bool(_) => f.write_str("boolean"),
                Value::Number(_) => f.write_str("number"),
                Value::String(_) => f.write_str("string"),
                Value::Array(_) => f.write_str("array"),
                Value::Object(_) => f.write_str("object"),
            }
        }
    }

    let value = ValueWrapper(Value::Array(vec![]));
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::from(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "array");
}

#[test]
fn test_fmt_value_object() {
    use std::fmt;
    use serde_json::Value;

    struct ValueWrapper(Value);

    impl fmt::Debug for ValueWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self.0 {
                Value::Null => f.write_str("null"),
                Value::Bool(_) => f.write_str("boolean"),
                Value::Number(_) => f.write_str("number"),
                Value::String(_) => f.write_str("string"),
                Value::Array(_) => f.write_str("array"),
                Value::Object(_) => f.write_str("object"),
            }
        }
    }

    let value = ValueWrapper(Value::Object(serde_json::Map::new()));
    let mut output = String::new();
    let result = value.fmt(&mut fmt::Formatter::from(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "object");
}

