// Answer 0

#[test]
fn test_insert_into_vacant_entry() {
    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn insert_unique(&mut self, hash: HashValue, key: K, value: V) -> OccupiedEntry<K, V> {
            self.data.push((key, value));
            OccupiedEntry::new(self, hash.0 as usize) // Using hash as an index for testing simplicity
        }

        fn get(&self, index: usize) -> &V {
            &self.data[index].1
        }
    }

    let mut entries = TestEntries::new();
    let key = String::from("test_key");
    let value = 42;
    let hash_value = HashValue(1);
    let mut vacant_entry = VacantEntry {
        map: RefMut {
            indices: &mut Indices, // Assume suitable initialization for simplicity
            entries: &mut entries,
        },
        hash: hash_value,
        key,
    };

    let result: &mut i32 = vacant_entry.insert(value);
    assert_eq!(*result, 42);
    assert_eq!(entries.data.len(), 1);
    assert_eq!(entries.get(0), &42); 
}

#[test]
#[should_panic]
fn test_insert_panic_on_key_duplicate() {
    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K: PartialEq, V> TestEntries<K, V> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn insert_unique(&mut self, _: HashValue, key: K, value: V) -> OccupiedEntry<K, V> {
            if self.data.iter().any(|(k, _)| *k == key) {
                panic!("Duplicate key insertion not allowed");
            }
            self.data.push((key, value));
            OccupiedEntry::new(self, self.data.len() - 1)
        }

        fn get(&self, index: usize) -> &V {
            &self.data[index].1
        }
    }

    let mut entries = TestEntries::new();
    let key = String::from("duplicate_key");
    let value = 99;
    let hash_value = HashValue(1);

    let mut vacant_entry = VacantEntry {
        map: RefMut {
            indices: &mut Indices, // Assume suitable initialization for simplicity
            entries: &mut entries,
        },
        hash: hash_value,
        key: key.clone(),
    };

    vacant_entry.insert(value);

    // Re-using the same key to trigger panic
    let another_vacant_entry = VacantEntry {
        map: RefMut {
            indices: &mut Indices, // Assume suitable initialization for simplicity
            entries: &mut entries,
        },
        hash: HashValue(2),
        key,
    };

    another_vacant_entry.insert(100);
}

