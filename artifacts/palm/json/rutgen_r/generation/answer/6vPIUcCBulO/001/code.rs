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

    fn index(&self, index: &str) -> &serde_json::Value {
        self.map.get(index).expect("Key not found")
    }
}

#[test]
fn test_index_existing_key() {
    let mut my_map = MyMap::new();
    my_map.map.insert("key1".to_string(), serde_json::json!(123));
    
    let value = my_map.index("key1");
    assert_eq!(*value, serde_json::json!(123));
}

#[test]
#[should_panic(expected = "Key not found")]
fn test_index_non_existing_key() {
    let my_map = MyMap::new();
    my_map.index("non_existing_key");
}

