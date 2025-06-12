// Answer 0

#[test]
fn test_shift_remove_entry_existing_key() {
    struct TestIndexMap {
        entries: Vec<(String, i32)>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                entries: vec![("key1".to_string(), 1), ("key2".to_string(), 2)],
            }
        }

        fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, i32)>
        where
            Q: ?Sized + std::hash::Hash + PartialEq<String>,
        {
            if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
                let (k, v) = self.entries.remove(pos);
                Some((k, v))
            } else {
                None
            }
        }
    }

    let mut map = TestIndexMap::new();
    let result = map.shift_remove_entry(&"key1");
    assert_eq!(result, Some(("key1".to_string(), 1)));
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0], ("key2".to_string(), 2));
}

#[test]
fn test_shift_remove_entry_non_existing_key() {
    struct TestIndexMap {
        entries: Vec<(String, i32)>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                entries: vec![("key1".to_string(), 1), ("key2".to_string(), 2)],
            }
        }

        fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, i32)>
        where
            Q: ?Sized + std::hash::Hash + PartialEq<String>,
        {
            if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
                let (k, v) = self.entries.remove(pos);
                Some((k, v))
            } else {
                None
            }
        }
    }

    let mut map = TestIndexMap::new();
    let result = map.shift_remove_entry(&"key3");
    assert_eq!(result, None);
    assert_eq!(map.entries.len(), 2);
}

