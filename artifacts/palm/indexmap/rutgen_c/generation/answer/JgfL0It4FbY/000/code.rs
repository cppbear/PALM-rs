// Answer 0

#[test]
fn test_shift_remove_entry() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            Bucket { hash: HashValue::from(0), key: 1, value: 10 },
            Bucket { hash: HashValue::from(1), key: 2, value: 20 },
            Bucket { hash: HashValue::from(2), key: 3, value: 30 },
        ],
    };

    // Simulate an occupied entry
    let index = hashbrown::hash_table::OccupiedEntry::from_index(1);
    let occupied_entry = OccupiedEntry::new(&mut test_entries, index);

    // Perform the operation
    let (removed_key, removed_value) = occupied_entry.shift_remove_entry();

    // Check the removed values
    assert_eq!(removed_key, 2);
    assert_eq!(removed_value, 20);

    // Check the remaining entries
    assert_eq!(test_entries.entries.len(), 2);
    assert_eq!(test_entries.entries[0].key, 1);
    assert_eq!(test_entries.entries[0].value, 10);
    assert_eq!(test_entries.entries[1].key, 3);
    assert_eq!(test_entries.entries[1].value, 30);
}

#[test]
fn test_shift_remove_entry_boundary() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            Bucket { hash: HashValue::from(0), key: 1, value: 10 },
        ],
    };

    // Simulate an occupied entry
    let index = hashbrown::hash_table::OccupiedEntry::from_index(0);
    let occupied_entry = OccupiedEntry::new(&mut test_entries, index);

    // Perform the operation
    let (removed_key, removed_value) = occupied_entry.shift_remove_entry();

    // Check the removed values
    assert_eq!(removed_key, 1);
    assert_eq!(removed_value, 10);

    // Check the remaining entries
    assert_eq!(test_entries.entries.len(), 0);
}

