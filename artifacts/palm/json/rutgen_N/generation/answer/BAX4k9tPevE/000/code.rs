// Answer 0

#[derive(Debug)]
struct CustomMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

impl CustomMap {
    fn new() -> Self {
        CustomMap {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: serde_json::Value) {
        self.map.insert(key, value);
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &serde_json::Value)>
    where
        String: std::borrow::Borrow<Q>,
        Q: ?Sized + Ord + Eq + std::hash::Hash,
    {
        self.map.get_key_value(key)
    }
}

#[test]
fn test_get_key_value_found() {
    let mut custom_map = CustomMap::new();
    custom_map.insert("key1".to_string(), serde_json::json!(42));

    let result = custom_map.get_key_value("key1");
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, &"key1".to_string());
    assert_eq!(result.unwrap().1, &serde_json::json!(42));
}

#[test]
fn test_get_key_value_not_found() {
    let custom_map = CustomMap::new();
    
    let result = custom_map.get_key_value("key_not_found");
    assert!(result.is_none());
}

#[test]
fn test_get_key_value_with_different_borrowed_form() {
    let mut custom_map = CustomMap::new();
    custom_map.insert("key2".to_string(), serde_json::json!("value"));

    let borrowed_key: &str = "key2";
    let result = custom_map.get_key_value(borrowed_key);
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, &"key2".to_string());
    assert_eq!(result.unwrap().1, &serde_json::json!("value"));
}

