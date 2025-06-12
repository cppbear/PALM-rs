// Answer 0

#[test]
fn test_insert_entry_occupied() {
    use hashbrown::HashMap;

    struct TestEntries<K, V> {
        map: HashMap<K, V>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.map.insert(key, value);
        }

        fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            self.map.get_mut(key)
        }

        fn occupied_entry<'a>(&'a mut self, key: K) -> OccupiedEntry<'a, K, V>
        where
            K: Eq + std::hash::Hash,
            V: Default,
        {
            self.map.insert(key, V::default());
            let index = self.map.len() - 1;
            OccupiedEntry::new(self, index)
        }
    }

    let mut entries = TestEntries::new();
    let key = "test_key";
    let value = "initial_value";
    entries.insert(key.to_string(), value.to_string());

    let entry = Entry::Occupied(entries.occupied_entry(key.to_string()));
    let new_value = "new_value";
    let occupied_entry = entry.insert_entry(new_value.to_string());

    assert_eq!(occupied_entry.get(), &new_value.to_string());
    assert_eq!(entries.get_mut(&key.to_string()).unwrap(), &new_value.to_string());
}

