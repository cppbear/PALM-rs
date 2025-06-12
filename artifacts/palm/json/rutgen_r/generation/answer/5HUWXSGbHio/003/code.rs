// Answer 0

#[test]
fn test_fmt_string_value() {
    use std::fmt;

    #[derive(Debug)]
    enum Value {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(Vec<(String, Value)>),
    }

    struct MyValue(Value);

    impl fmt::Display for MyValue {
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

    // Test with a string value
    let string_value = MyValue(Value::String("test".to_string()));
    assert_eq!(format!("{}", string_value), "string");

    // Test with an empty string value
    let empty_string_value = MyValue(Value::String("".to_string()));
    assert_eq!(format!("{}", empty_string_value), "string");
}

