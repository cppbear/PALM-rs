// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestEntry<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestEntry<K, V> {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn add_entry(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }

        fn get_mut(&mut self, idx: usize) -> &mut V {
            &mut self.entries[idx].1
        }
    }

    let mut entries = TestEntry::new();
    let key = "key1";
    let value = "value1";
    entries.add_entry(key.to_string(), value.to_string());

    let mut occupied_entry = OccupiedEntry::new(&mut entries, hashbrown::hash_table::OccupiedEntry { index: 0 });

    let entry = Entry::Occupied(occupied_entry);
    let returned_value = entry.or_insert("default_value".to_string());

    assert_eq!(returned_value, &mut value.to_string());
    assert_eq!(entries.entries.len(), 1);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestEntry<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestEntry<K, V> {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) -> &mut V {
            self.entries.push((key, value));
            &mut self.entries.last_mut().unwrap().1
        }
    }

    let mut entries = TestEntry::new();
    let key = "key2";

    let vacant_entry = VacantEntry {
        map: RefMut::default(), // Assuming RefMut is initialized properly
        hash: HashValue::default(), // Assuming HashValue has a default
        key: key.to_string(),
    };

    let entry = Entry::Vacant(vacant_entry);
    let returned_value = entry.or_insert("new_value".to_string());

    assert_eq!(returned_value, &mut "new_value".to_string());
    assert_eq!(entries.entries.len(), 1);
}

