// Answer 0

#[derive(Serialize)]
struct Value {
    data: ValueType,
}

#[derive(Serialize)]
enum ValueType {
    String(String),
}

#[test]
fn test_serialize_string() {
    let value = Value {
        data: ValueType::String("test string".to_string()),
    };

    let mut buffer = serde_json::Serializer::new(String::new());
    let result = value.serialize(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer.into_inner(), "\"test string\"");
}

#[test]
fn test_serialize_empty_string() {
    let value = Value {
        data: ValueType::String("".to_string()),
    };

    let mut buffer = serde_json::Serializer::new(String::new());
    let result = value.serialize(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer.into_inner(), "\"\""); 
}

