// Answer 0

#[test]
fn test_remove_entry_basic() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                data: indexmap::IndexMap::new(),
            }
        }
        
        fn insert(&mut self, key: i32, value: String) {
            self.data.insert(key, value);
        }

        fn remove_entry(&mut self, key: i32) -> Option<(i32, String)> {
            if self.data.contains_key(&key) {
                let entry = self.data.swap_remove_entry(&key);
                entry.map(|(k, v)| (k.clone(), v.clone())) // Use clone to match return type
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let (key, value) = map.remove_entry(1).expect("Should have removed the key");
    assert_eq!(key, 1);
    assert_eq!(value, "one");
}

#[test]
fn test_remove_entry_non_existent() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                data: indexmap::IndexMap::new(),
            }
        }
        
        fn insert(&mut self, key: i32, value: String) {
            self.data.insert(key, value);
        }

        fn remove_entry(&mut self, key: i32) -> Option<(i32, String)> {
            if self.data.contains_key(&key) {
                let entry = self.data.swap_remove_entry(&key);
                entry.map(|(k, v)| (k.clone(), v.clone())) // Use clone to match return type
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());

    let result = map.remove_entry(2);
    assert!(result.is_none(), "Expected None when removing a non-existent key.");
}

