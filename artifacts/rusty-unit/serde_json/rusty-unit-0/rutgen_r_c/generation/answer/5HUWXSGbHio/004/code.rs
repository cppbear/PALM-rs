// Answer 0

#[test]
fn test_fmt_value_null() {
    let value = Value::Null;
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = write!(&mut output, "{}", type_instance);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "null");
}

#[test]
fn test_fmt_value_bool() {
    let value = Value::Bool(true);
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = write!(&mut output, "{}", type_instance);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "boolean");
}

#[test]
fn test_fmt_value_number() {
    let value = Value::Number(Number::from(12.5));
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = write!(&mut output, "{}", type_instance);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "number");
}

#[test]
fn test_fmt_value_string() {
    let value = Value::String(String::from("a string"));
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = write!(&mut output, "{}", type_instance);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "string");
}

#[test]
fn test_fmt_value_array() {
    let value = Value::Array(vec![Value::String(String::from("an array"))]);
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = write!(&mut output, "{}", type_instance);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "array");
}

#[test]
fn test_fmt_value_object() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(map);
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = write!(&mut output, "{}", type_instance);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "object");
}

