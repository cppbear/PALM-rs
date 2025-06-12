// Answer 0

#[test]
fn test_fmt_number_value() {
    use std::fmt;
    use serde_json::Value;

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

    // Test case for Value::Number
    let number_value = ValueWrapper(Value::Number(serde_json::Number::from(42)));
    let mut output = String::new();
    let result = write!(&mut output, "{}", number_value);
    
    assert!(result.is_ok());
    assert_eq!(output, "number");
}

