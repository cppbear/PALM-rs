// Answer 0

#[test]
fn test_value_string_debug() {
    use crate::value::Value;

    let value = Value::String(String::from("Hello, World!"));
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", value);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "String(\"Hello, World!\")");
}

#[test]
fn test_value_bool_debug() {
    use crate::value::Value;

    let value = Value::Bool(true);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", value);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Bool(true)");
}

#[test]
fn test_value_null_debug() {
    use crate::value::Value;

    let value = Value::Null;
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", value);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Null");
}

#[test]
fn test_value_number_debug() {
    use crate::value::{Value, Number};

    let number = Number { n: 42 };
    let value = Value::Number(number);
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", value);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Number(42)");
}

