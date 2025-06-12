// Answer 0

#[derive(Default)]
struct MyMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

impl MyMap {
    pub fn insert(&mut self, k: String, v: serde_json::Value) -> Option<serde_json::Value> {
        self.map.insert(k, v)
    }
}

#[test]
fn test_insert_new_key() {
    let mut my_map = MyMap::default();
    let result = my_map.insert("key1".to_string(), serde_json::json!(10));
    assert_eq!(result, None);
    assert_eq!(my_map.map.get("key1"), Some(&serde_json::json!(10)));
}

#[test]
fn test_insert_existing_key() {
    let mut my_map = MyMap::default();
    my_map.insert("key1".to_string(), serde_json::json!(10));
    let result = my_map.insert("key1".to_string(), serde_json::json!(20));
    assert_eq!(result, Some(serde_json::json!(10)));
    assert_eq!(my_map.map.get("key1"), Some(&serde_json::json!(20)));
} 

#[test]
fn test_insert_empty_key() {
    let mut my_map = MyMap::default();
    let result = my_map.insert("".to_string(), serde_json::json!(10));
    assert_eq!(result, None);
    assert_eq!(my_map.map.get(""), Some(&serde_json::json!(10)));
}

#[test]
fn test_insert_key_with_null_value() {
    let mut my_map = MyMap::default();
    let result = my_map.insert("key1".to_string(), serde_json::Value::Null);
    assert_eq!(result, None);
    assert_eq!(my_map.map.get("key1"), Some(&serde_json::Value::Null));
}

