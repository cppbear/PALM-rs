// Answer 0

#[test]
fn test_or_default_occupied() {
    struct TestEntry {
        value: i32,
    }

    struct TestMap {
        entries: std::collections::HashMap<String, TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: std::collections::HashMap::new(),
            }
        }

        fn entry(&mut self, key: String) -> Entry<TestMap, String, TestEntry> {
            if self.entries.contains_key(&key) {
                Entry::Occupied(OccupiedEntry { map: self, key })
            } else {
                Entry::Vacant(VacantEntry { map: self, key })
            }
        }
    }

    enum Entry<'a, K, V> {
        Occupied(OccupiedEntry<'a, K, V>),
        Vacant(VacantEntry<'a, K, V>),
    }

    struct OccupiedEntry<'a, K, V> {
        map: &'a mut TestMap,
        key: K,
    }

    struct VacantEntry<'a, K, V> {
        map: &'a mut TestMap,
        key: K,
    }

    impl<'a> OccupiedEntry<'a, String, TestEntry> {
        fn into_mut(self) -> &'a mut TestEntry {
            self.map.entries.get_mut(&self.key).unwrap()
        }
    }

    impl<'a> VacantEntry<'a, String, TestEntry> {
        fn insert(self, value: TestEntry) -> &'a mut TestEntry {
            self.map.entries.insert(self.key, value);
            self.map.entries.get_mut(&self.key).unwrap()
        }
    }

    let mut map = TestMap::new();
    map.entries.insert("key1".to_string(), TestEntry { value: 10 });

    // Test the occupied case
    let entry = map.entry("key1".to_string());
    let value_ref = entry.or_default();

    assert_eq!(value_ref.value, 10);
    
    // Modifying the value to ensure mutability
    value_ref.value += 5;

    // Check if the value reflects the modification
    assert_eq!(map.entries.get("key1").unwrap().value, 15);
}

#[test]
fn test_or_default_vacant() {
    struct TestEntry {
        value: i32,
    }

    struct TestMap {
        entries: std::collections::HashMap<String, TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: std::collections::HashMap::new(),
            }
        }

        fn entry(&mut self, key: String) -> Entry<TestMap, String, TestEntry> {
            if self.entries.contains_key(&key) {
                Entry::Occupied(OccupiedEntry { map: self, key })
            } else {
                Entry::Vacant(VacantEntry { map: self, key })
            }
        }
    }

    enum Entry<'a, K, V> {
        Occupied(OccupiedEntry<'a, K, V>),
        Vacant(VacantEntry<'a, K, V>),
    }

    struct OccupiedEntry<'a, K, V> {
        map: &'a mut TestMap,
        key: K,
    }

    struct VacantEntry<'a, K, V> {
        map: &'a mut TestMap,
        key: K,
    }

    impl<'a> OccupiedEntry<'a, String, TestEntry> {
        fn into_mut(self) -> &'a mut TestEntry {
            self.map.entries.get_mut(&self.key).unwrap()
        }
    }

    impl<'a> VacantEntry<'a, String, TestEntry> {
        fn insert(self, value: TestEntry) -> &'a mut TestEntry {
            self.map.entries.insert(self.key, value);
            self.map.entries.get_mut(&self.key).unwrap()
        }
    }

    let mut map = TestMap::new();

    // Test the vacant case
    let entry = map.entry("key2".to_string());
    let value_ref = entry.or_default();

    // Since this is a vacant entry, the default should be used
    assert_eq!(value_ref.value, 0);

    // Check if the new entry is present in the map
    assert!(map.entries.contains_key("key2"));
}

