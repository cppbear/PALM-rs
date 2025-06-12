// Answer 0

#[test]
fn test_insert_entry() {
    use indexmap::IndexMap;

    struct TestEntry<'a, K, V> {
        map: &'a mut IndexMap<K, V>,
        hash: u64,
        key: K,
    }

    struct TestOccupiedEntry<'a, K, V> {
        entry: &'a mut TestEntry<K, V>,
    }

    impl<'a, K, V> TestEntry<'a, K, V> {
        pub fn insert_entry(self, value: V) -> TestOccupiedEntry<'a, K, V> {
            let Self { map, hash, key } = self;
            map.insert(key, value);
            TestOccupiedEntry { entry: self }
        }
    }

    let mut map = IndexMap::new();
    let entry = TestEntry { map: &mut map, hash: 42, key: "test_key" };
    
    let occupied_entry = entry.insert_entry("test_value");

    assert_eq!(occupied_entry.entry.map.get("test_key"), Some(&"test_value"));
}

#[test]
fn test_insert_entry_overwrite() {
    use indexmap::IndexMap;

    struct TestEntry<'a, K, V> {
        map: &'a mut IndexMap<K, V>,
        hash: u64,
        key: K,
    }

    struct TestOccupiedEntry<'a, K, V> {
        entry: &'a mut TestEntry<K, V>,
    }

    impl<'a, K, V> TestEntry<'a, K, V> {
        pub fn insert_entry(self, value: V) -> TestOccupiedEntry<'a, K, V> {
            let Self { map, hash, key } = self;
            map.insert(key, value);
            TestOccupiedEntry { entry: self }
        }
    }

    let mut map = IndexMap::new();
    let entry1 = TestEntry { map: &mut map, hash: 42, key: "test_key" };
    entry1.insert_entry("first_value");
    let entry2 = TestEntry { map: &mut map, hash: 42, key: "test_key" };
    
    let occupied_entry = entry2.insert_entry("second_value");

    assert_eq!(occupied_entry.entry.map.get("test_key"), Some(&"second_value"));
}

