// Answer 0

#[test]
fn test_remove_existing_key() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
    let result = map.remove("key1");
    assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    assert!(map.get("key1").is_none());
}

#[test]
fn test_remove_non_existing_key() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
    let result = map.remove("key2");
    assert_eq!(result, None);
}

#[test]
fn test_remove_with_preserve_order() {
    #[cfg(feature = "preserve_order")]
    {
        let mut map = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
        map.insert("key2".to_string(), serde_json::Value::String("value2".to_string()));
        
        let result = map.remove("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
        assert!(map.get("key1").is_none());
        assert_eq!(map.len(), 1);
    }
}

#[test]
fn test_remove_preserve_order_empty_map() {
    #[cfg(feature = "preserve_order")]
    {
        let mut map = serde_json::Map::new();
        let result = map.remove("key1");
        assert_eq!(result, None);
    }
}

