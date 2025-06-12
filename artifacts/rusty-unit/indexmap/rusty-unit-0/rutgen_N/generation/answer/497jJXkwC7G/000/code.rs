// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use std::collections::hash_map::RandomState;
    use std::hash::Hash;

    struct TestEntry {
        key: String,
        value: String,
    }

    struct TestMap {
        entries: indexmap::IndexMap<String, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: indexmap::IndexMap::new(),
            }
        }

        fn entry(&mut self, key: String) -> indexmap::map::Entry<String, String, RandomState> {
            self.entries.entry(key)
        }
    }

    let mut map = TestMap::new();
    let (key, value) = map.entry("key1".to_string()).or_insert("default_key".to_string(), "default_value".to_string());
    assert_eq!(*key, "default_key");
    assert_eq!(*value, "default_value");
}

#[test]
fn test_or_insert_occupied_entry() {
    use std::collections::hash_map::RandomState;
    use std::hash::Hash;

    struct TestEntry {
        key: String,
        value: String,
    }

    struct TestMap {
        entries: indexmap::IndexMap<String, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: indexmap::IndexMap::new(),
            }
        }

        fn entry(&mut self, key: String) -> indexmap::map::Entry<String, String, RandomState> {
            self.entries.entry(key)
        }
    }

    let mut map = TestMap::new();
    map.entries.insert("key1".to_string(), "existing_value".to_string());
    let (key, value) = map.entry("key1".to_string()).or_insert("default_key".to_string(), "default_value".to_string());
    assert_eq!(*key, "key1");
    assert_eq!(*value, "existing_value");
}

