// Answer 0

#[test]
fn test_remove_valid_entry() {
    struct TestMap {
        entries: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.entries.insert(key, value);
        }

        fn get(&self, key: i32) -> Option<&String> {
            self.entries.get(&key)
        }

        fn remove_entry(self) -> Option<(i32, String)> {
            if let Some((key, value)) = self.entries.last_key_value() {
                self.entries.swap_remove(*key);
                return Some((*key, value.clone()));
            }
            None
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let removed = map.remove_entry();
    assert!(removed.is_some());
    let (key, value) = removed.unwrap();
    assert_eq!(key, 2);
    assert_eq!(value, "two");

    // Ensuring the entry is removed
    assert!(map.get(2).is_none());
}

#[test]
#[should_panic]
fn test_remove_from_empty_map() {
    struct TestMap {
        entries: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: indexmap::IndexMap::new(),
            }
        }

        fn remove_entry(self) -> Option<(i32, String)> {
            if let Some((key, value)) = self.entries.last_key_value() {
                self.entries.swap_remove(*key);
                return Some((*key, value.clone()));
            }
            panic!("Attempted to remove from an empty map");
        }
    }

    let map = TestMap::new();
    map.remove_entry(); // This should panic
}

#[test]
fn test_remove_multiple_entries() {
    struct TestMap {
        entries: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.entries.insert(key, value);
        }

        fn remove_entry(self) -> Option<(i32, String)> {
            if let Some((key, value)) = self.entries.last_key_value() {
                self.entries.swap_remove(*key);
                return Some((*key, value.clone()));
            }
            None
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    for _ in 0..3 {
        let removed = map.remove_entry();
        assert!(removed.is_some());
    }

    // At this point, the map should be empty
    assert!(map.remove_entry().is_none());
}

