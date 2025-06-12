// Answer 0

#[test]
fn test_serialize_null() {
    let value = Value::Null;
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let output = serializer.into_inner();
    assert_eq!(output, b"null");
}

#[test]
fn test_serialize_bool() {
    let value = Value::Bool(true);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let output = serializer.into_inner();
    assert_eq!(output, b"true");
}

#[test]
fn test_serialize_number() {
    let number = Number { n: 123.0 }; // Assuming N can be a float
    let value = Value::Number(number);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let output = serializer.into_inner();
    assert_eq!(output, b"123");
}

#[test]
fn test_serialize_string() {
    let value = Value::String("a string".to_owned());
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let output = serializer.into_inner();
    assert_eq!(output, b"\"a string\"");
}

#[test]
fn test_serialize_array() {
    let value = Value::Array(vec![Value::Number(Number { n: 1.0 }), Value::Number(Number { n: 2.0 })]);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let output = serializer.into_inner();
    assert_eq!(output, b"[1,2]");
}

#[test]
fn test_serialize_object() {
    let mut map = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));
    let value = Value::Object(map);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let output = serializer.into_inner();
    assert_eq!(output, b"{\"key\":\"value\"}");
}

