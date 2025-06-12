// Answer 0

#[test]
fn test_get_full_mut_existing_key() {
    use std::hash::Hash;
    use std::collections::HashSet;

    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct Map<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K: Hash + Eq, V> Map<K, V> {
        fn new() -> Self {
            Map { entries: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<K>,
        {
            self.entries.iter().position(|entry| &entry.key == key)
        }

        fn as_entries_mut(&mut self) -> &mut Vec<Entry<K, V>> {
            &mut self.entries
        }

        pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
        where
            Q: ?Sized + Hash + Equivalent<K>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some((i, &entry.key, &mut entry.value))
            } else {
                None
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct TestKey(usize);

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map = Map::new();
    map.insert(TestKey(1), "value1");
    map.insert(TestKey(2), "value2");

    if let Some((index, key_ref, value_ref)) = map.get_full_mut(&TestKey(1)) {
        assert_eq!(index, 0);
        assert_eq!(key_ref, &TestKey(1));
        assert_eq!(*value_ref, "value1");
        *value_ref = "updated_value";
    } else {
        panic!("Expected Some, but got None.");
    }

    assert_eq!(map.entries[0].value, "updated_value");
}

