// Answer 0

#[test]
fn test_as_object_mut_with_object() {
    let mut value = Value::Object(Map { map: Default::default() });
    assert!(value.as_object_mut().is_some());
}

#[test]
fn test_as_object_mut_with_non_object() {
    let mut value = Value::Bool(true);
    assert!(value.as_object_mut().is_none());
}

#[test]
fn test_as_object_mut_after_modification() {
    let mut value = Value::Object(Map { map: Default::default() });
    if let Some(map_mut) = value.as_object_mut() {
        // Imagine an operation that modifies the map
        map_mut.map.insert(String::from("key"), Value::Bool(false));
    }
    
    assert!(value.as_object_mut().is_some());
}

#[test]
fn test_as_object_mut_with_empty_object() {
    let mut value = Value::Object(Map { map: Default::default() });
    if let Some(map_mut) = value.as_object_mut() {
        // Clear the map
        map_mut.map = Default::default();
    }
    
    assert!(value.as_object_mut().is_some());
}

