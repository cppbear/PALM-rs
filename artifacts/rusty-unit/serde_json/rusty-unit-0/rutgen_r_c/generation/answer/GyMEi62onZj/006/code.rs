// Answer 0

#[test]
fn test_value_fmt_null() {
    use crate::value::Value;

    let value = Value::Null;
    let mut formatter = fmt::Formatter::new();

    let result = value.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "Null");
}

#[test]
fn test_value_fmt_bool() {
    use crate::value::Value;

    let value_true = Value::Bool(true);
    let mut formatter_true = fmt::Formatter::new();

    let result_true = value_true.fmt(&mut formatter_true);
    assert!(result_true.is_ok());
    assert_eq!(formatter_true.to_string(), "Bool(true)");

    let value_false = Value::Bool(false);
    let mut formatter_false = fmt::Formatter::new();

    let result_false = value_false.fmt(&mut formatter_false);
    assert!(result_false.is_ok());
    assert_eq!(formatter_false.to_string(), "Bool(false)");
}

#[test]
fn test_value_fmt_number() {
    use crate::value::{Value, Number};
    use core::fmt;

    struct TestNumber;
    impl Debug for TestNumber {
        fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(_formatter, "TestNumber")
        }
    }
    impl Number for TestNumber {}

    let value = Value::Number(TestNumber);
    let mut formatter = fmt::Formatter::new();

    let result = value.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "TestNumber");
}

#[test]
fn test_value_fmt_string() {
    use crate::value::Value;

    let value = Value::String(String::from("a string"));
    let mut formatter = fmt::Formatter::new();

    let result = value.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "String(\"a string\")");
}

#[test]
fn test_value_fmt_array() {
    use crate::value::{Value, Map};
    
    let array = vec![Value::Null, Value::Bool(true)];
    let value = Value::Array(array);
    let mut formatter = fmt::Formatter::new();

    let result = value.fmt(&mut formatter);
    assert!(result.is_ok());
    assert!(formatter.to_string().starts_with("Array "));
}

#[test]
fn test_value_fmt_object() {
    use crate::value::{Value, Map};

    let mut map = Map::new();
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    let value = Value::Object(map);
    let mut formatter = fmt::Formatter::new();

    let result = value.fmt(&mut formatter);
    assert!(result.is_ok());
    assert!(formatter.to_string().starts_with("Object "));
}

