// Answer 0

#[test]
fn test_value_fmt_null() {
    let value = Value::Null;
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = value.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(output, "Null");
}

#[test]
fn test_value_fmt_bool_true() {
    let value = Value::Bool(true);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = value.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(output, "Bool(true)");
}

#[test]
fn test_value_fmt_bool_false() {
    let value = Value::Bool(false);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = value.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(output, "Bool(false)");
}

#[test]
fn test_value_fmt_array() {
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = value.fmt(formatter);
    assert!(result.is_ok());
    assert!(output.starts_with("Array "));
}

#[test]
fn test_value_fmt_number() {
    let number = Number { n: 5 }; // Assuming `n` can be initialized as an integer for testing
    let value = Value::Number(number);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = value.fmt(formatter);
    assert!(result.is_ok());
    // assuming the Debug implementation for Number outputs "5"
    assert_eq!(output, "Number(5)");
}

#[test]
fn test_value_fmt_string() {
    let string_value = Value::String(String::from("test string"));
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = string_value.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(output, "String(\"test string\")");
}

#[test]
fn test_value_fmt_object() {
    let object_value = Value::Object(Map { map: MapImpl::new() }); // Assuming MapImpl can be initialized
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = object_value.fmt(formatter);
    assert!(result.is_ok());
    assert!(output.starts_with("Object "));
}

