// Answer 0

#[test]
fn test_new_with_empty_map() {
    // Create an empty Map
    let map = Map { map: std::collections::BTreeMap::new() };
    
    // Call the new function
    let deserializer = MapRefDeserializer::new(&map);
    
    // Assert the iter is empty and value is None
    assert_eq!(std::iter::IntoIterator::into_iter(deserializer.iter).count(), 0);
    assert!(deserializer.value.is_none());
}

#[test]
fn test_new_with_single_element_map() {
    // Create a Map with a single Value
    let mut data = std::collections::BTreeMap::new();
    data.insert("key".to_string(), Value::String("value".to_string()));
    let map = Map { map: data };
    
    // Call the new function
    let deserializer = MapRefDeserializer::new(&map);
    
    // Assert the iter is not empty and value is None
    assert_eq!(std::iter::IntoIterator::into_iter(deserializer.iter).count(), 1);
    assert!(deserializer.value.is_none());
}

#[test]
fn test_new_with_multiple_elements_map() {
    // Create a Map with multiple Values
    let mut data = std::collections::BTreeMap::new();
    data.insert("key1".to_string(), Value::Bool(true));
    data.insert("key2".to_string(), Value::Number(Number::from(42)));
    let map = Map { map: data };
    
    // Call the new function
    let deserializer = MapRefDeserializer::new(&map);
    
    // Assert the iter is not empty and value is None
    assert_eq!(std::iter::IntoIterator::into_iter(deserializer.iter).count(), 2);
    assert!(deserializer.value.is_none());
}

