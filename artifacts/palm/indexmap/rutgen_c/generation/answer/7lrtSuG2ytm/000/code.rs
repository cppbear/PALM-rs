// Answer 0

#[test]
fn test_key_mut() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

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

    let mut test_entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::from(1), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::from(2), key: 2, value: "two".to_string() },
        ],
    };

    let occupied_entry_index = hashbrown::hash_table::OccupiedEntry::new(0);
    let mut occupied_entry = OccupiedEntry::new(&mut test_entries, occupied_entry_index);

    let key_mut = occupied_entry.key_mut();
    *key_mut = 3;

    assert_eq!(test_entries.entries[0].key, 3);
}

#[test]
#[should_panic]
fn test_key_mut_out_of_bounds() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

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

    let mut test_entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::from(1), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::from(2), key: 2, value: "two".to_string() },
        ],
    };

    let occupied_entry_index = hashbrown::hash_table::OccupiedEntry::new(2); // Invalid index
    let mut occupied_entry = OccupiedEntry::new(&mut test_entries, occupied_entry_index);

    occupied_entry.key_mut(); // This should panic
}

