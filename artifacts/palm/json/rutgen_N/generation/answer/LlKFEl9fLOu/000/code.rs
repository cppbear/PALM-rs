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

    fn index_mut(&mut self, index: &String) -> &mut serde_json::Value {
        self.map.get_mut(index).expect("no entry found for key")
    }
}

#[test]
fn test_index_mut_existing_key() {
    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), serde_json::json!(42));

    let value = my_map.index_mut(&"key1".to_string());
    *value = serde_json::json!(100);

    assert_eq!(my_map.map.get("key1").unwrap(), &serde_json::json!(100));
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_non_existing_key() {
    let mut my_map = MyMap::new();
    my_map.index_mut(&"non_existing_key".to_string());
}

