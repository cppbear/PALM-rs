// Answer 0

#[test]
fn test_into_mut() {
    struct MockEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> super::Entries for MockEntries<K, V> {
        type Entry = Bucket<K, V>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut mock_entries = MockEntries {
        entries: vec![
            Bucket {
                hash: HashValue::from(1),
                key: "key1",
                value: 10,
            },
            Bucket {
                hash: HashValue::from(2),
                key: "key2",
                value: 20,
            },
        ],
    };

    let index = 0;
    let occupied_entry = OccupiedEntry::new(&mut mock_entries, hash_table::OccupiedEntry::new(index));
    
    let value_mut: &mut i32 = occupied_entry.into_mut();
    *value_mut = 30;

    assert_eq!(mock_entries.as_entries()[index].value, 30);
}

