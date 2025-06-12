// Answer 0

#[test]
fn test_serialize_str_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_str("");
    assert_eq!(result, Ok(Value::String(String::from(""))));
}

#[test]
fn test_serialize_str_simple() {
    let serializer = Serializer;
    let result = serializer.serialize_str("hello");
    assert_eq!(result, Ok(Value::String(String::from("hello"))));
}

#[test]
fn test_serialize_str_special_characters() {
    let serializer = Serializer;
    let result = serializer.serialize_str("!@#$%^&*()_+");
    assert_eq!(result, Ok(Value::String(String::from("!@#$%^&*()_+"))));
}

#[test]
fn test_serialize_str_unicode() {
    let serializer = Serializer;
    let result = serializer.serialize_str("你好");
    assert_eq!(result, Ok(Value::String(String::from("你好"))));
}

#[test]
fn test_serialize_str_long_string() {
    let serializer = Serializer;
    let long_string = "a".repeat(1000);
    let result = serializer.serialize_str(&long_string);
    assert_eq!(result, Ok(Value::String(long_string)));
}

