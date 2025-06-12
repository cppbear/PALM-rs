// Answer 0

#[test]
fn test_fmt_with_object() {
    use serde_json::Value;
    use std::fmt;

    struct ValueWrapper(Value);
    
    impl fmt::Display for ValueWrapper {
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
    
    let object_value = Value::Object(serde_json::Map::new());
    let wrapped_object = ValueWrapper(object_value);
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", wrapped_object);
    
    assert!(result.is_ok());
    assert_eq!(output, "object");
}

#[test]
fn test_fmt_with_null() {
    use serde_json::Value;
    use std::fmt;

    struct ValueWrapper(Value);

    impl fmt::Display for ValueWrapper {
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

    let null_value = Value::Null;
    let wrapped_null = ValueWrapper(null_value);
    let mut output = String::new();

    let result = write!(&mut output, "{}", wrapped_null);

    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_fmt_with_boolean() {
    use serde_json::Value;
    use std::fmt;

    struct ValueWrapper(Value);

    impl fmt::Display for ValueWrapper {
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

    let boolean_value = Value::Bool(true);
    let wrapped_boolean = ValueWrapper(boolean_value);
    let mut output = String::new();

    let result = write!(&mut output, "{}", wrapped_boolean);

    assert!(result.is_ok());
    assert_eq!(output, "boolean");
} 

#[test]
fn test_fmt_with_number() {
    use serde_json::Value;
    use std::fmt;

    struct ValueWrapper(Value);

    impl fmt::Display for ValueWrapper {
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

    let number_value = Value::Number(serde_json::Number::from(42));
    let wrapped_number = ValueWrapper(number_value);
    let mut output = String::new();

    let result = write!(&mut output, "{}", wrapped_number);

    assert!(result.is_ok());
    assert_eq!(output, "number");
} 

#[test]
fn test_fmt_with_string() {
    use serde_json::Value;
    use std::fmt;

    struct ValueWrapper(Value);

    impl fmt::Display for ValueWrapper {
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

    let string_value = Value::String("hello".to_string());
    let wrapped_string = ValueWrapper(string_value);
    let mut output = String::new();

    let result = write!(&mut output, "{}", wrapped_string);

    assert!(result.is_ok());
    assert_eq!(output, "string");
} 

#[test]
fn test_fmt_with_array() {
    use serde_json::Value;
    use std::fmt;

    struct ValueWrapper(Value);

    impl fmt::Display for ValueWrapper {
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

    let array_value = Value::Array(vec![]);
    let wrapped_array = ValueWrapper(array_value);
    let mut output = String::new();

    let result = write!(&mut output, "{}", wrapped_array);

    assert!(result.is_ok());
    assert_eq!(output, "array");
}

