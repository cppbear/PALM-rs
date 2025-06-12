// Answer 0

#[derive(Debug)]
struct TestMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

impl TestMap {
    fn new() -> Self {
        TestMap {
            map: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: serde_json::Value) {
        self.map.insert(key, value);
    }

    fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &serde_json::Value)>
    where
        String: std::borrow::Borrow<Q>,
        Q: ?Sized + Ord + Eq + std::hash::Hash,
    {
        self.map.get_key_value(key)
    }
}

#[test]
fn test_get_key_value_existing_key() {
    let mut test_map = TestMap::new();
    test_map.insert("key1".to_string(), serde_json::json!(1));
    
    let result = test_map.get_key_value("key1");
    assert!(result.is_some());
    let (key, value) = result.unwrap();
    assert_eq!(key, &"key1".to_string());
    assert_eq!(value, &serde_json::json!(1));
}

#[test]
fn test_get_key_value_non_existing_key() {
    let mut test_map = TestMap::new();
    test_map.insert("key1".to_string(), serde_json::json!(1));
    
    let result = test_map.get_key_value("key2");
    assert!(result.is_none());
}

#[test]
fn test_get_key_value_empty_map() {
    let test_map = TestMap::new();
    
    let result = test_map.get_key_value("anykey");
    assert!(result.is_none());
}

#[test]
fn test_get_key_value_with_borrowed_str() {
    let mut test_map = TestMap::new();
    test_map.insert("key1".to_string(), serde_json::json!(1));
    
    let key: &str = "key1";
    let result = test_map.get_key_value(key);
    assert!(result.is_some());
    let (found_key, found_value) = result.unwrap();
    assert_eq!(found_key, &"key1".to_string());
    assert_eq!(found_value, &serde_json::json!(1));
}

#[test]
fn test_get_key_value_different_ordering() {
    let mut test_map = TestMap::new();
    test_map.insert("key1".to_string(), serde_json::json!(1));
    test_map.insert("key2".to_string(), serde_json::json!(2));

    let key: &str = "key2"; // Testing existing key
    let result = test_map.get_key_value(key);
    assert!(result.is_some());
    
    let key_non_existing: &str = "key3"; // Testing non-existing key
    let result_non_existing = test_map.get_key_value(key_non_existing);
    assert!(result_non_existing.is_none());
}

