// Answer 0

#[test]
fn test_serialize_str_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_str("");
    assert_eq!(result, Ok(Value::String(String::from(""))));
}

#[test]
fn test_serialize_str_non_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_str("test");
    assert_eq!(result, Ok(Value::String(String::from("test"))));
}

#[test]
fn test_serialize_str_unicode() {
    let serializer = Serializer;
    let result = serializer.serialize_str("你好");
    assert_eq!(result, Ok(Value::String(String::from("你好"))));
}

#[test]
fn test_serialize_str_special_characters() {
    let serializer = Serializer;
    let result = serializer.serialize_str("!@#$%^&*()");
    assert_eq!(result, Ok(Value::String(String::from("!@#$%^&*()"))));
}

