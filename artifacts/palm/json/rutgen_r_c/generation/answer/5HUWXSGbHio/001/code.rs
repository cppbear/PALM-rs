// Answer 0

#[test]
fn test_fmt_with_value_null() {
    let value = Value::Null;
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = type_instance.fmt(&mut output).is_ok();
    assert_eq!(result, true);
    assert_eq!(String::from_utf8(output).unwrap(), "null");
}

#[test]
fn test_fmt_with_value_bool() {
    let value = Value::Bool(true);
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = type_instance.fmt(&mut output).is_ok();
    assert_eq!(result, true);
    assert_eq!(String::from_utf8(output).unwrap(), "boolean");
}

#[test]
fn test_fmt_with_value_number() {
    let value = Value::Number(Number::from(12.5));
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = type_instance.fmt(&mut output).is_ok();
    assert_eq!(result, true);
    assert_eq!(String::from_utf8(output).unwrap(), "number");
}

#[test]
fn test_fmt_with_value_string() {
    let value = Value::String(String::from("a string"));
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = type_instance.fmt(&mut output).is_ok();
    assert_eq!(result, true);
    assert_eq!(String::from_utf8(output).unwrap(), "string");
}

#[test]
fn test_fmt_with_value_array() {
    let value = Value::Array(vec![Value::Null, Value::Bool(false)]);
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = type_instance.fmt(&mut output).is_ok();
    assert_eq!(result, true);
    assert_eq!(String::from_utf8(output).unwrap(), "array");
}

#[test]
fn test_fmt_with_value_object() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(map);
    let type_instance = Type(&value);
    let mut output = vec![];
    let result = type_instance.fmt(&mut output).is_ok();
    assert_eq!(result, true);
    assert_eq!(String::from_utf8(output).unwrap(), "object");
}

