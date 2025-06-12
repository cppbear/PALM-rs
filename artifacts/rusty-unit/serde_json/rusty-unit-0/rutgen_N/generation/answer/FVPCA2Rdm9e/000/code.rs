// Answer 0

#[derive(Debug)]
struct TestMap {
    map: std::collections::HashMap<String, i32>,
}

impl TestMap {
    pub fn new() -> Self {
        TestMap {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: i32) {
        self.map.insert(key, value);
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<String, i32> {
        self.map.keys()
    }
}

#[test]
fn test_keys_empty_map() {
    let test_map = TestMap::new();
    let keys: Vec<&String> = test_map.keys().collect();
    assert!(keys.is_empty());
}

#[test]
fn test_keys_single_entry() {
    let mut test_map = TestMap::new();
    test_map.insert("key1".to_string(), 1);
    let keys: Vec<&String> = test_map.keys().collect();
    assert_eq!(keys, vec![&"key1".to_string()]);
}

#[test]
fn test_keys_multiple_entries() {
    let mut test_map = TestMap::new();
    test_map.insert("key1".to_string(), 1);
    test_map.insert("key2".to_string(), 2);
    test_map.insert("key3".to_string(), 3);
    let keys: Vec<&String> = test_map.keys().collect();
    assert_eq!(keys, vec![&"key1".to_string(), &"key2".to_string(), &"key3".to_string()]);
}

