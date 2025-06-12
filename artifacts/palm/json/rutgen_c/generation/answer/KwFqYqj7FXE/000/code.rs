// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::String("value2".to_string()));
    
    let result = map.remove_entry(&"key1");
    
    assert_eq!(result, Some(("key1".to_string(), Value::String("value1".to_string()))));
    assert!(!map.contains_key(&"key1"));
    assert!(map.contains_key(&"key2"));
}

#[test]
fn test_remove_entry_non_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let result = map.remove_entry(&"non_existing_key");
    
    assert_eq!(result, None);
    assert!(map.contains_key(&"key1"));
}

#[test]
fn test_remove_entry_preserve_order() {
    #[cfg(feature = "preserve_order")]
    {
        let mut map = Map::with_capacity(2);
        map.insert("key1".to_string(), Value::String("value1".to_string()));
        map.insert("key2".to_string(), Value::String("value2".to_string()));
        
        let result = map.remove_entry(&"key1");
        
        assert_eq!(result, Some(("key1".to_string(), Value::String("value1".to_string()))));
        assert!(!map.contains_key(&"key1"));
        assert!(map.contains_key(&"key2"));
    }
}

#[test]
fn test_remove_entry_boundary_conditions() {
    let mut map = Map::new();
    let result = map.remove_entry(&"key1");
    
    assert_eq!(result, None);
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

