// Answer 0

#[test]
fn test_index_into_with_existing_key() {
    let key = String::from("key");
    let mut object = Value::Object(Map::new());
    object.insert(key.clone(), Value::Bool(true));

    assert_eq!(key.index_into(&object), Some(&Value::Bool(true)));
}

#[test]
fn test_index_into_with_non_existing_key() {
    let key = String::from("non_existing_key");
    let object = Value::Object(Map::new());

    assert_eq!(key.index_into(&object), None);
}

#[test]
#[should_panic]
fn test_index_into_with_empty_key() {
    let key = String::from("");
    let object = Value::Object(Map::new());
    
    // This should panic
    key.index_into(&object);
}

#[test]
fn test_index_into_with_null_value() {
    let key = String::from("null_key");
    let mut object = Value::Object(Map::new());
    object.insert(key.clone(), Value::Null);

    assert_eq!(key.index_into(&object), Some(&Value::Null));
}

#[test]
fn test_index_into_with_mixed_values() {
    let key = String::from("mixed_key");
    let mut object = Value::Object(Map::new());
    object.insert(key.clone(), Value::Number(Number::from(3.14)));
    
    assert_eq!(key.index_into(&object), Some(&Value::Number(Number::from(3.14))));
}

