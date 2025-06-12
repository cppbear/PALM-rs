// Answer 0

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

struct CustomFormatter;

impl std::fmt::Write for CustomFormatter {
    fn write_str(&mut self, _: &str) -> std::fmt::Result {
        Ok(())
    }
}

struct TestValue(Box<Value>);

impl TestValue {
    fn new(value: Value) -> Self {
        TestValue(Box::new(value))
    }

    fn fmt(&self, formatter: &mut CustomFormatter) -> std::fmt::Result {
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

#[test]
fn test_fmt_array() {
    let array_value = Value::Array(vec![Value::Number(1), Value::Bool(true)]);
    let test_value = TestValue::new(array_value);
    let mut formatter = CustomFormatter;

    let result = test_value.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_empty_array() {
    let empty_array_value = Value::Array(vec![]);
    let test_value = TestValue::new(empty_array_value);
    let mut formatter = CustomFormatter;

    let result = test_value.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_other_types_not_matching() {
    let null_value = Value::Null;
    let test_value = TestValue::new(null_value);
    let mut formatter = CustomFormatter;

    let result = test_value.fmt(&mut formatter);
    assert!(result.is_ok());
}

