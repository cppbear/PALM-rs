// Answer 0

#[test]
fn test_iter_empty_map() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
            self.map.iter()
        }
    }

    let empty_map = TestMap {
        map: std::collections::HashMap::new(),
    };

    let iter = empty_map.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_entry_map() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
            self.map.iter()
        }
    }

    let mut single_entry_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    single_entry_map.map.insert("key1".to_string(), "value1".to_string());

    let mut iter = single_entry_map.iter();
    assert_eq!(iter.next(), Some((&"key1".to_string(), &"value1".to_string())));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_entries_map() {
    struct TestMap {
        map: std::collections::HashMap<String, String>,
    }

    impl TestMap {
        pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
            self.map.iter()
        }
    }

    let mut multiple_entries_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    multiple_entries_map.map.insert("key1".to_string(), "value1".to_string());
    multiple_entries_map.map.insert("key2".to_string(), "value2".to_string());
    multiple_entries_map.map.insert("key3".to_string(), "value3".to_string());

    let mut iter = multiple_entries_map.iter();
    assert_eq!(iter.next(), Some((&"key1".to_string(), &"value1".to_string())));
    assert_eq!(iter.next(), Some((&"key2".to_string(), &"value2".to_string())));
    assert_eq!(iter.next(), Some((&"key3".to_string(), &"value3".to_string())));
    assert!(iter.next().is_none());
}

