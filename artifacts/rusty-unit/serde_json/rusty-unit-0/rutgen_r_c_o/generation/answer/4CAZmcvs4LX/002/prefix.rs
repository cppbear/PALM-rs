// Answer 0

#[test]
fn test_serialize_element_with_bool() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &true; 
    let _ = serializer.serialize_element(value);
}

#[test]
fn test_serialize_element_with_number() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &42; 
    let _ = serializer.serialize_element(value);
}

#[test]
fn test_serialize_element_with_string() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &"Hello"; 
    let _ = serializer.serialize_element(value);
}

#[test]
fn test_serialize_element_with_array() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &vec![Value::Number(1.into()), Value::String("two".to_string())]; 
    let _ = serializer.serialize_element(value);
}

#[test]
fn test_serialize_element_with_object() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(2.into()));
    let value = &Value::Object(map); 
    let _ = serializer.serialize_element(value);
}

#[test]
fn test_serialize_element_with_large_vector() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &vec![Value::I64(i as i64) for i in 0..1000]; 
    let _ = serializer.serialize_element(value);
}

#[test]
fn test_serialize_element_with_empty_vector() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value: Vec<Value> = vec![]; 
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_with_complex_object() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let mut map = Map::new();
    for i in 0..100 {
        map.insert(format!("key{}", i), Value::Bool(i % 2 == 0));
    }
    let value = &Value::Object(map);
    let _ = serializer.serialize_element(value);
}

