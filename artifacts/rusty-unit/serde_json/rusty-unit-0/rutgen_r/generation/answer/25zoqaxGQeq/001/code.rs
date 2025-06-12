// Answer 0

#[test]
fn test_serialize_char_valid() {
    let value = 'a';
    let result = serialize_char(value);
    assert_eq!(result, Ok(Value::String("a".to_string())));
}

#[test]
fn test_serialize_char_boundary() {
    let value = ' ';
    let result = serialize_char(value);
    assert_eq!(result, Ok(Value::String(" ".to_string())));
}

#[test]
fn test_serialize_char_special() {
    let value = 'ß';
    let result = serialize_char(value);
    assert_eq!(result, Ok(Value::String("ß".to_string())));
}

#[test]
fn test_serialize_char_unicode() {
    let value = '中';
    let result = serialize_char(value);
    assert_eq!(result, Ok(Value::String("中".to_string())));
}

#[test]
fn test_serialize_char_max() {
    let value = '\u{10FFFF}';
    let result = serialize_char(value);
    assert_eq!(result, Ok(Value::String(String::from("\u{10FFFF}"))));
}

