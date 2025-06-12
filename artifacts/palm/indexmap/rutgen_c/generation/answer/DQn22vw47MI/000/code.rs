// Answer 0

#[test]
fn test_insert_vacant_entry() {
    struct TestEntries<K, V> {
        data: std::collections::HashMap<K, V>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            TestEntries {
                data: std::collections::HashMap::new(),
            }
        }

        fn insert_unique(&mut self, hash: HashValue, key: K, value: V) -> OccupiedEntry<K, V> {
            self.data.insert(key, value);
            // Simplified index simulation
            let index = self.data.len() - 1;
            OccupiedEntry::new(self, hash_table::OccupiedEntry::new(index))
        }
    }

    let mut entries = TestEntries::new();
    let key = String::from("test_key");
    let value = 42;
    let hash_value = HashValue(0);
    
    let vacant_entry = VacantEntry {
        map: RefMut {
            indices: &mut Indices::new(), // Placeholder, assume Indices struct exists
            entries: &mut entries,
        },
        hash: hash_value,
        key,
    };
    
    let mut_ref = vacant_entry.insert(value);
    assert_eq!(*mut_ref, 42);
}

#[test]
fn test_insert_overwriting_value() {
    struct TestEntries<K, V> {
        data: std::collections::HashMap<K, V>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            TestEntries {
                data: std::collections::HashMap::new(),
            }
        }

        fn insert_unique(&mut self, hash: HashValue, key: K, value: V) -> OccupiedEntry<K, V> {
            self.data.insert(key.clone(), value);
            // Simplified index simulation
            let index = self.data.len() - 1;
            OccupiedEntry::new(self, hash_table::OccupiedEntry::new(index))
        }
    }

    let mut entries = TestEntries::new();
    let key = String::from("test_key");
    let value1 = 42;
    let value2 = 84;
    let hash_value = HashValue(0);
    
    let vacant_entry = VacantEntry {
        map: RefMut {
            indices: &mut Indices::new(), // Placeholder, assume Indices struct exists
            entries: &mut entries,
        },
        hash: hash_value,
        key: key.clone(),
    };
    
    let mut_ref1 = vacant_entry.insert(value1);
    assert_eq!(*mut_ref1, 42);
    
    let occupied_entry = entries.insert_unique(hash_value, key.clone(), value2);
    let mut_ref2 = occupied_entry.insert(value2);
    assert_eq!(*mut_ref2, 84);
}

