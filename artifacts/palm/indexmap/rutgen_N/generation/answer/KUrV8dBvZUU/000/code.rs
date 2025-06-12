// Answer 0

#[test]
fn test_entry_insertion() {
    use indexmap::IndexMap;

    struct TestEntry<K, V> {
        map: IndexMap<K, V>
    }

    impl<K, V> TestEntry<K, V> {
        fn new() -> Self {
            TestEntry { map: IndexMap::new() }
        }

        fn insert_entry(&mut self, key: K, value: V) {
            self.map.entry(key).or_insert(value);
        }
    }

    let mut entry_test = TestEntry::new();
    entry_test.insert_entry("key1", "value1");

    assert_eq!(entry_test.map.get("key1"), Some(&"value1"));
}

#[test]
fn test_entry_existing_key() {
    use indexmap::IndexMap;

    struct TestEntry<K, V> {
        map: IndexMap<K, V>
    }

    impl<K, V> TestEntry<K, V> {
        fn new() -> Self {
            TestEntry { map: IndexMap::new() }
        }

        fn insert_entry(&mut self, key: K, value: V) {
            self.map.entry(key).or_insert(value);
        }
    }

    let mut entry_test = TestEntry::new();
    entry_test.insert_entry("key1", "value1");
    entry_test.insert_entry("key1", "value2");

    assert_eq!(entry_test.map.get("key1"), Some(&"value1"));
}

#[test]
fn test_entry_non_existent_key() {
    use indexmap::IndexMap;

    struct TestEntry<K, V> {
        map: IndexMap<K, V>
    }

    impl<K, V> TestEntry<K, V> {
        fn new() -> Self {
            TestEntry { map: IndexMap::new() }
        }

        fn insert_entry(&mut self, key: K, value: V) {
            self.map.entry(key).or_insert(value);
        }
    }

    let mut entry_test = TestEntry::new();
    entry_test.insert_entry("key2", "value2");
    
    assert_eq!(entry_test.map.get("key1"), None);
}

