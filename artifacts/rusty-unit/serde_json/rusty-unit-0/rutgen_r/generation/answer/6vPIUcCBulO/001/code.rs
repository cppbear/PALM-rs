// Answer 0

#[derive(Debug)]
struct TestMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

impl TestMap {
    fn new() -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert("key1".to_string(), serde_json::json!(1));
        map.insert("key2".to_string(), serde_json::json!("value2"));
        map.insert("key3".to_string(), serde_json::json!(true));
        Self { map }
    }

    fn index(&self, index: &str) -> &serde_json::Value {
        self.map.get(index).unwrap()
    }
}

#[test]
fn test_index_existing_key() {
    let test_map = TestMap::new();
    let value = test_map.index("key1");
    assert_eq!(value, &serde_json::json!(1));
}

#[test]
fn test_index_non_existing_key() {
    let test_map = TestMap::new();
    let result = std::panic::catch_unwind(|| {
        test_map.index("key_non_existing");
    });
    assert!(result.is_err());
}

#[test]
fn test_index_key2() {
    let test_map = TestMap::new();
    let value = test_map.index("key2");
    assert_eq!(value, &serde_json::json!("value2"));
}

#[test]
fn test_index_key3() {
    let test_map = TestMap::new();
    let value = test_map.index("key3");
    assert_eq!(value, &serde_json::json!(true));
}

