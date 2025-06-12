// Answer 0

#[test]
fn test_get_mut() {
    struct TestEntries<K, V> {
        vec: Vec<Bucket<K, V>>,
    }

    impl<K, V> Entries for TestEntries<K, V> {
        type Entry = Bucket<K, V>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.vec
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.vec
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.vec
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.vec);
        }
    }

    // Test case with a valid index
    let mut entries = TestEntries {
        vec: vec![
            Bucket { hash: HashValue::new(1), key: "key1", value: 10 },
            Bucket { hash: HashValue::new(2), key: "key2", value: 20 },
        ],
    };

    {
        let index = 1;
        let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::from_index(&mut entries.vec, index));
        let value_ref = occupied_entry.get_mut();
        *value_ref += 5;
        assert_eq!(entries.vec[index].value, 25);
    }

    // Test case triggering panic by accessing an invalid index
    let invalid_index = 2; // Out of range index
    let result = std::panic::catch_unwind(|| {
        let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::from_index(&mut entries.vec, invalid_index));
        let _ = occupied_entry.get_mut();
    });
    assert!(result.is_err());
}

