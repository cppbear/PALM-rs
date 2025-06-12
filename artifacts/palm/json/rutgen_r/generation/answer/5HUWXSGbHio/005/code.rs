// Answer 0

#[test]
fn test_fmt_bool() {
    use serde_json::Value;
    use std::fmt::{self, Write};

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

    let true_value = TestValue(Value::Bool(true));
    let false_value = TestValue(Value::Bool(false));

    // Testing a boolean true
    let mut output = String::new();
    true_value.fmt(&mut output).unwrap();
    assert_eq!(output, "boolean");

    // Resetting output for the next test
    output.clear();

    // Testing a boolean false
    false_value.fmt(&mut output).unwrap();
    assert_eq!(output, "boolean");
}

