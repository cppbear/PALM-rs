// Answer 0

#[test]
fn test_clear() {
    let mut map = Map::new();
    assert_eq!(map.len(), 0);
    
    // Insert some values
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number::from(10)));
    map.insert("key3".to_string(), Value::String("value".to_string()));
    
    // Verify that the length is now greater than 0
    assert!(map.len() > 0);
    
    // Clear the map
    map.clear();
    
    // Assert that the map is empty after clearing
    assert_eq!(map.len(), 0);
}

#[test]
fn test_clear_on_empty_map() {
    let mut map = Map::new();
    
    // Clearing an already empty map
    map.clear();
    
    // Assert that the map remains empty
    assert_eq!(map.len(), 0);
}

