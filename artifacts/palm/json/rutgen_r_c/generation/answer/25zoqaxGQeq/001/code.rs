// Answer 0

#[test]
fn test_serialize_char_valid_char() {
    let serializer = Serializer;
    let result = serializer.serialize_char('a');
    assert_eq!(result, Ok(Value::String(String::from("a"))));
}

#[test]
fn test_serialize_char_valid_unicode_char() {
    let serializer = Serializer;
    let result = serializer.serialize_char('汉');
    assert_eq!(result, Ok(Value::String(String::from("汉"))));
}

#[test]
fn test_serialize_char_valid_special_char() {
    let serializer = Serializer;
    let result = serializer.serialize_char('!'); 
    assert_eq!(result, Ok(Value::String(String::from("!"))));
}

#[test]
fn test_serialize_char_valid_whitespace_char() {
    let serializer = Serializer;
    let result = serializer.serialize_char(' '); 
    assert_eq!(result, Ok(Value::String(String::from(" "))));
}

#[test]
fn test_serialize_char_valid_digit_char() {
    let serializer = Serializer;
    let result = serializer.serialize_char('1'); 
    assert_eq!(result, Ok(Value::String(String::from("1"))));
}

