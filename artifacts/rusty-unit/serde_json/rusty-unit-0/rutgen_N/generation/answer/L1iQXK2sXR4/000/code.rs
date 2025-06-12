// Answer 0

#[test]
fn test_remove_existing_key_with_preserve_order() {
    use serde_json::{Map, Value};
    use std::borrow::Borrow;
    use std::hash::Hash;

    #[cfg(feature = "preserve_order")]
    {
        let mut map: Map<String, Value> = Map::new();
        map.insert("key1".to_string(), Value::from(1));
        map.insert("key2".to_string(), Value::from(2));
        
        let removed_value = map.remove::<str>("key1");
        assert_eq!(removed_value, Some(Value::from(1)));
        assert_eq!(map.len(), 1);
        assert!(map.get("key1").is_none());
        assert_eq!(map.get("key2"), Some(&Value::from(2)));
    }
}

#[test]
fn test_remove_non_existing_key_with_preserve_order() {
    use serde_json::{Map, Value};
    #[cfg(feature = "preserve_order")]
    {
        let mut map: Map<String, Value> = Map::new();
        map.insert("key1".to_string(), Value::from(1));
        
        let removed_value = map.remove::<str>("key2");
        assert_eq!(removed_value, None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key1"), Some(&Value::from(1)));
    }
}

#[test]
fn test_remove_existing_key_without_preserve_order() {
    use serde_json::{Map, Value};
    use std::borrow::Borrow;
    use std::hash::Hash;

    #[cfg(not(feature = "preserve_order"))]
    {
        let mut map: Map<String, Value> = Map::new();
        map.insert("key1".to_string(), Value::from(1));
        map.insert("key2".to_string(), Value::from(2));
        
        let removed_value = map.remove::<str>("key1");
        assert_eq!(removed_value, Some(Value::from(1)));
        assert_eq!(map.len(), 1);
        assert!(map.get("key1").is_none());
        assert_eq!(map.get("key2"), Some(&Value::from(2)));
    }
}

#[test]
fn test_remove_non_existing_key_without_preserve_order() {
    use serde_json::{Map, Value};
    #[cfg(not(feature = "preserve_order"))]
    {
        let mut map: Map<String, Value> = Map::new();
        map.insert("key1".to_string(), Value::from(1));
        
        let removed_value = map.remove::<str>("key2");
        assert_eq!(removed_value, None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key1"), Some(&Value::from(1)));
    }
}

