// Answer 0

#[derive(Debug)]
struct MyMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

impl MyMap {
    fn new() -> Self {
        MyMap {
            map: std::collections::HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: String, value: serde_json::Value) {
        self.map.insert(key, value);
    }
    
    fn index(&self, index: &String) -> &serde_json::Value {
        self.map.get(index).expect("Index not found")
    }
}

#[test]
fn test_index_existing_key() {
    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), serde_json::json!(42));
    
    let value = my_map.index(&"key1".to_string());
    assert_eq!(value, &serde_json::json!(42));
}

#[test]
#[should_panic(expected = "Index not found")]
fn test_index_non_existing_key() {
    let my_map = MyMap::new();
    let _value = my_map.index(&"non_existing_key".to_string());
}

