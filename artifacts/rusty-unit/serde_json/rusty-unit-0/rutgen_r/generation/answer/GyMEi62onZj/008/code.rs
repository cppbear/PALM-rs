// Answer 0

#[test]
fn test_fmt_null_variants() {
    let value_null = Value::Null;
    let mut formatter = Formatter::new();
    let result = value_null.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.output, "Null");
}

#[test]
fn test_fmt_bool_variants() {
    let value_true = Value::Bool(true);
    let mut formatter_true = Formatter::new();
    let result_true = value_true.fmt(&mut formatter_true);
    assert!(result_true.is_ok());
    assert_eq!(formatter_true.output, "Bool(true)");

    let value_false = Value::Bool(false);
    let mut formatter_false = Formatter::new();
    let result_false = value_false.fmt(&mut formatter_false);
    assert!(result_false.is_ok());
    assert_eq!(formatter_false.output, "Bool(false)");
}

#[test]
fn test_fmt_number_variants() {
    let value_number = Value::Number(42);
    let mut formatter = Formatter::new();
    let result = value_number.fmt(&mut formatter);
    assert!(result.is_ok()); // Assuming Debug::fmt works without panicking for this number
}

#[test]
fn test_fmt_string_variants() {
    let value_string = Value::String(String::from("Hello"));
    let mut formatter = Formatter::new();
    let result = value_string.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.output, "String(\"Hello\")");
}

#[test]
fn test_fmt_array_variants() {
    let value_array = Value::Array(vec![Value::Null, Value::Bool(true)]);
    let mut formatter = Formatter::new();
    let result = value_array.fmt(&mut formatter);
    assert!(result.is_ok());
    assert!(formatter.output.starts_with("Array "));
}

#[test]
fn test_fmt_object_variants() {
    let value_object = Value::Object(BTreeMap::from([(String::from("key"), Value::Number(1))]));
    let mut formatter = Formatter::new();
    let result = value_object.fmt(&mut formatter);
    assert!(result.is_ok());
    assert!(formatter.output.starts_with("Object "));
}

