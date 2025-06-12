// Answer 0

#[test]
fn test_serialize_value_null() {
    let value = Value::Null;
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let serialized = String::from_utf8(serializer.into_inner()).unwrap();
    assert_eq!(serialized, "null");
}

#[test]
fn test_serialize_value_bool_true() {
    let value = Value::Bool(true);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let serialized = String::from_utf8(serializer.into_inner()).unwrap();
    assert_eq!(serialized, "true");
}

#[test]
fn test_serialize_value_bool_false() {
    let value = Value::Bool(false);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let serialized = String::from_utf8(serializer.into_inner()).unwrap();
    assert_eq!(serialized, "false");
}

#[test]
fn test_serialize_value_number() {
    let number = Number { n: 12 };  // Assuming N is a type representing a number
    let value = Value::Number(number);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let serialized = String::from_utf8(serializer.into_inner()).unwrap();
    assert_eq!(serialized, "12"); // Adjust expected output based on your definition of Number
}

#[test]
fn test_serialize_value_string() {
    let value = Value::String("test string".to_string());
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let serialized = String::from_utf8(serializer.into_inner()).unwrap();
    assert_eq!(serialized, "\"test string\"");
}

#[test]
fn test_serialize_value_array() {
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let serialized = String::from_utf8(serializer.into_inner()).unwrap();
    assert_eq!(serialized, "[true,false]");
}

#[test]
fn test_serialize_value_object() {
    let mut map = Map::new();
    map.insert("key".to_string(), Value::Bool(true));
    let value = Value::Object(map);
    let mut serializer = serde_json::Serializer::new(Vec::new());
    let result = value.serialize(&mut serializer);
    assert!(result.is_ok());
    let serialized = String::from_utf8(serializer.into_inner()).unwrap();
    assert_eq!(serialized, "{\"key\":true}");
}

