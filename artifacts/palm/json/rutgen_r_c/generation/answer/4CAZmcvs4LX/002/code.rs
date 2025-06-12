// Answer 0

#[test]
fn test_serialize_element_with_bool() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let result = serializer.serialize_element(&true);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_element_with_null() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let result = serializer.serialize_element(&Value::Null);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_element_with_number() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let result = serializer.serialize_element(&Value::Number(serde_json::Number::from(123)));
    assert!(result.is_ok());
}

#[test]
fn test_serialize_element_with_string() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let result = serializer.serialize_element(&Value::String("test".to_string()));
    assert!(result.is_ok());
}

#[test]
fn test_serialize_element_with_array() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let result = serializer.serialize_element(&Value::Array(vec![Value::String("element".to_string())]));
    assert!(result.is_ok());
}

#[test]
fn test_serialize_element_with_object() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let mut object = Map::new();
    object.insert("key".to_string(), Value::String("value".to_string()));
    
    let result = serializer.serialize_element(&Value::Object(object));
    assert!(result.is_ok());
}

