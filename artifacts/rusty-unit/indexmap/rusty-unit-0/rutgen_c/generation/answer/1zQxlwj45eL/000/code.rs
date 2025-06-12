// Answer 0

#[test]
fn test_shift_insert_within_bounds() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn insert(&mut self, index: usize, bucket: Bucket<K, V>) {
            self.entries.insert(index, bucket);
        }

        fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {
            let end = self.indices.len();
            assert!(index <= end);
            self.indices.insert(index, hash.0);
            self.insert(index, Bucket { hash, key, value });
        }
    }

    let mut indices = vec![];
    let mut entries: Vec<Bucket<i32, String>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: "One".to_string() },
        Bucket { hash: HashValue(2), key: 2, value: "Two".to_string() },
    ];

    let mut map = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry { map, hash: HashValue(3), key: 3 };

    let value_ref = vacant_entry.shift_insert(1, "Three".to_string());
    
    assert_eq!(value_ref, &mut entries[1].value);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[1].key, 3);
}

#[should_panic]
#[test]
fn test_shift_insert_out_of_bounds() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn insert(&mut self, index: usize, bucket: Bucket<K, V>) {
            self.entries.insert(index, bucket);
        }

        fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {
            let end = self.indices.len();
            assert!(index <= end);
            self.indices.insert(index, hash.0);
            self.insert(index, Bucket { hash, key, value });
        }
    }

    let mut indices = vec![];
    let mut entries: Vec<Bucket<i32, String>> = vec![];

    let mut map = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry { map, hash: HashValue(3), key: 3 };

    // Attempting to insert out of bounds
    vacant_entry.shift_insert(2, "Four".to_string());
}

