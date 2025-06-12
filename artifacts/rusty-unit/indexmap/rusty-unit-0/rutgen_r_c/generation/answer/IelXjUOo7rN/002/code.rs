// Answer 0

#[test]
fn test_or_default_occupied() {
    struct TestMap<K, V> {
        data: std::collections::HashMap<K, V>,
    }

    impl<K: std::hash::Hash + Eq, V: Default> TestMap<K, V> {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.data.insert(key, value);
        }

        fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            self.data.get_mut(key)
        }
    }

    struct Entries<'a, K, V> {
        map: &'a mut TestMap<K, V>,
        keys: Vec<K>,
    }

    impl<'a, K: std::hash::Hash + Eq, V: Default> Entries<'a, K, V> {
        fn new(map: &'a mut TestMap<K, V>) -> Self {
            Self {
                map,
                keys: Vec::new(),
            }
        }

        fn occupied_entry(&mut self, key: K) -> Option<OccupiedEntry<'a, K, V>> {
            self.keys.push(key.clone());
            self.map.get_mut(&key).map(|value| {
                OccupiedEntry::new(self, key)
            })
        }
    }

    let mut test_map = TestMap::new();
    let key = "key1";
    test_map.insert(key, 42);

    let mut entries = Entries::new(&mut test_map);
    
    if let Some(entry) = entries.occupied_entry(key) {
        let value_ref = entry.into_mut();
        assert_eq!(*value_ref, 42);
    } else {
        panic!("Should have found the occupied entry");
    }
}

#[test]
fn test_or_default_vacant() {
    struct TestMap<K, V> {
        data: std::collections::HashMap<K, V>,
    }

    impl<K: std::hash::Hash + Eq, V: Default> TestMap<K, V> {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.data.insert(key, value);
        }

        fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            self.data.get_mut(key)
        }
    }

    struct VacantEntry<'a, K, V> {
        map: &'a mut TestMap<K, V>,
        key: K,
    }

    impl<'a, K: std::hash::Hash + Eq, V: Default> VacantEntry<'a, K, V> {
        fn insert(self, value: V) -> &'a mut V {
            self.map.insert(self.key, value);
            self.map.get_mut(&self.key).unwrap()
        }
    }

    struct Entries<'a, K, V> {
        map: &'a mut TestMap<K, V>,
    }

    impl<'a, K: std::hash::Hash + Eq, V: Default> Entries<'a, K, V> {
        fn new(map: &'a mut TestMap<K, V>) -> Self {
            Self { map }
        }

        fn vacant_entry(&mut self, key: K) -> VacantEntry<'a, K, V> {
            VacantEntry { map: self.map, key }
        }
    }

    let mut test_map = TestMap::new();
    
    let key = "key2";

    let mut entries = Entries::new(&mut test_map);
    let vacant_entry = entries.vacant_entry(key);
    let default_value_ref = vacant_entry.insert(V::default());

    assert_eq!(*default_value_ref, Default::default());
}

