// Answer 0

#[test]
fn test_values_mut_withNoElements() {
    let mut map = Map::new();
    let mut values_mut_iterator = map.values_mut();
    assert!(values_mut_iterator.iter.len() == 0);
}

#[test]
fn test_values_mut_withSingleElement() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let mut values_mut_iterator = map.values_mut();
    assert_eq!(values_mut_iterator.iter.len(), 1);
    assert_eq!(values_mut_iterator.iter.next(), Some(&mut Value::Bool(true)));
}

#[test]
fn test_values_mut_withMultipleElements() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Number(1.into()));
    map.insert("key2".to_string(), Value::String("value".to_string()));
    
    let mut values_mut_iterator = map.values_mut();
    assert_eq!(values_mut_iterator.iter.len(), 2);
    assert_eq!(values_mut_iterator.iter.next(), Some(&mut Value::Number(1.into())));
    assert_eq!(values_mut_iterator.iter.next(), Some(&mut Value::String("value".to_string())));
}

#[test]
fn test_values_mut_afterModification() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Bool(false));
    
    let mut values_mut_iterator = map.values_mut();
    assert_eq!(values_mut_iterator.iter.len(), 2);
    
    // Modify the value
    if let Some(value) = values_mut_iterator.iter.next() {
        *value = Value::Bool(true);
    }

    let mut modified_values = map.values();
    assert_eq!(modified_values.iter.next(), Some(&Value::Bool(true)));
    assert_eq!(modified_values.iter.next(), Some(&Value::Bool(false)));
}

