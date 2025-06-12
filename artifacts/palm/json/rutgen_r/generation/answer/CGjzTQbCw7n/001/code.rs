// Answer 0

#[test]
fn test_serialize_str_valid_input() {
    let value = "test string";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String("test string".to_owned())));
}

#[test]
fn test_serialize_str_empty_string() {
    let value = "";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String("".to_owned())));
}

#[test]
fn test_serialize_str_whitespace() {
    let value = "   ";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String("   ".to_owned())));
}

#[test]
fn test_serialize_str_special_characters() {
    let value = "!@#$%^&*()_+-=~";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String("!@#$%^&*()_+-=~".to_owned())));
}

#[test]
fn test_serialize_str_unicode_characters() {
    let value = "你好世界";
    let result = serialize_str(value);
    assert_eq!(result, Ok(Value::String("你好世界".to_owned())));
}

