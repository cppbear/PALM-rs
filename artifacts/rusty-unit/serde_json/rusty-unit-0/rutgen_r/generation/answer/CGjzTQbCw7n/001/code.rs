// Answer 0

#[test]
fn test_serialize_str_valid_input() {
    let value = "test string";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String(value.to_owned())));
}

#[test]
fn test_serialize_str_empty_string() {
    let value = "";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String(value.to_owned())));
}

#[test]
fn test_serialize_str_special_characters() {
    let value = "line1\nline2\tTabbed";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String(value.to_owned())));
}

#[test]
fn test_serialize_str_unicode_characters() {
    let value = "こんにちは";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String(value.to_owned())));
}

#[test]
fn test_serialize_str_numeric_string() {
    let value = "12345";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String(value.to_owned())));
}

