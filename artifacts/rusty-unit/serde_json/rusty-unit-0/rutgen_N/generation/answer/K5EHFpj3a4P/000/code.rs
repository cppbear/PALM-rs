// Answer 0

#[test]
fn test_as_bool_true() {
    use serde_json::Value;
    
    let value = Value::Bool(true);
    assert_eq!(value.as_bool(), Some(true));
}

#[test]
fn test_as_bool_false() {
    use serde_json::Value;
    
    let value = Value::Bool(false);
    assert_eq!(value.as_bool(), Some(false));
}

#[test]
fn test_as_bool_not_boolean() {
    use serde_json::Value;
    
    let value_string = Value::String("false".to_string());
    assert_eq!(value_string.as_bool(), None);
    
    let value_number = Value::Number(serde_json::Number::from(0));
    assert_eq!(value_number.as_bool(), None);
    
    let value_array = Value::Array(vec![]);
    assert_eq!(value_array.as_bool(), None);
    
    let value_object = Value::Object(serde_json::map::Map::new());
    assert_eq!(value_object.as_bool(), None);
}

