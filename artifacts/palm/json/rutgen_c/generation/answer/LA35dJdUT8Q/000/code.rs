// Answer 0

#[test]
fn test_as_object_with_object_value() {
    struct DummyMap {
        map: std::collections::HashMap<String, Value>,
    }
    
    let mut map = DummyMap { map: std::collections::HashMap::new() };
    map.map.insert(
        "nested".to_string(), 
        Value::Bool(true)
    );
  
    let value = Value::Object(Map { map: map.map });
    assert!(value.as_object().is_some());
}

#[test]
fn test_as_object_with_non_object_value() {
    let value = Value::Array(vec![Value::String("an".to_string()), Value::String("array".to_string())]);
    assert!(value.as_object().is_none());
}

#[test]
fn test_as_object_with_null_value() {
    let value = Value::Null;
    assert!(value.as_object().is_none());
}

#[test]
fn test_as_object_with_boolean_value() {
    let value = Value::Bool(false);
    assert!(value.as_object().is_none());
}

#[test]
fn test_as_object_with_number_value() {
    let value = Value::Number(Number { n: 5 }); // Assuming Number takes an inner type that represents a number
    assert!(value.as_object().is_none());
}

#[test]
fn test_as_object_with_string_value() {
    let value = Value::String("a string".to_string());
    assert!(value.as_object().is_none());
}

