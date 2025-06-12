// Answer 0

#[test]
fn test_serialize_null() {
    use serde_json::Value;
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;
    use std::io::Cursor;

    let value = Value::Null;
    let serializer = Serializer::with_formatter(Cursor::new(Vec::new()));
    
    let result = value.serialize(serializer);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"null");
}

#[test]
fn test_serialize_bool() {
    use serde_json::Value;
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;
    use std::io::Cursor;

    let value_true = Value::Bool(true);
    let value_false = Value::Bool(false);
    let serializer_true = Serializer::with_formatter(Cursor::new(Vec::new()));
    let serializer_false = Serializer::with_formatter(Cursor::new(Vec::new()));
    
    let result_true = value_true.serialize(serializer_true);
    assert!(result_true.is_ok());
    let serialized_true = result_true.unwrap();
    assert_eq!(serialized_true, b"true");
    
    let result_false = value_false.serialize(serializer_false);
    assert!(result_false.is_ok());
    let serialized_false = result_false.unwrap();
    assert_eq!(serialized_false, b"false");
}

#[test]
fn test_serialize_string() {
    use serde_json::Value;
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;
    use std::io::Cursor;

    let value = Value::String("test string".to_string());
    let serializer = Serializer::with_formatter(Cursor::new(Vec::new()));
    
    let result = value.serialize(serializer);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"\"test string\"");
}

#[test]
fn test_serialize_number() {
    use serde_json::Value;
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;
    use std::io::Cursor;

    // Assuming Number can be directly represented as a float for testing
    let value = Value::Number(serde_json::Number::from(12.5));
    let serializer = Serializer::with_formatter(Cursor::new(Vec::new()));
    
    let result = value.serialize(serializer);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"12.5");
}

#[test]
fn test_serialize_array() {
    use serde_json::Value;
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;
    use std::io::Cursor;

    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let serializer = Serializer::with_formatter(Cursor::new(Vec::new()));
    
    let result = value.serialize(serializer);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"[true,false]");
}

#[test]
fn test_serialize_object() {
    use serde_json::Value;
    use serde_json::Map;
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;
    use std::io::Cursor;

    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::String("value".to_string()));

    let value = Value::Object(map);
    let serializer = Serializer::with_formatter(Cursor::new(Vec::new()));
    
    let result = value.serialize(serializer);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"{\"key1\":true,\"key2\":\"value\"}");
}

