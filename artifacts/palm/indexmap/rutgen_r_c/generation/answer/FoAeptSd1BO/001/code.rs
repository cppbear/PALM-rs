// Answer 0

#[test]
fn test_get_valid_entry() {
    struct MockEntries {
        entries: Vec<Bucket<String, i32>>,
    }

    impl Entries for MockEntries {
        type Entry = Bucket<String, i32>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F) where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = MockEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: "key1".to_string(), value: 10 },
            Bucket { hash: HashValue(2), key: "key2".to_string(), value: 20 },
        ],
    };

    let index = hash_table::OccupiedEntry::new(&mut entries.entries, 0);
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    assert_eq!(occupied_entry.get(), &10);
}

#[test]
#[should_panic]
fn test_get_invalid_index() {
    struct MockEntries {
        entries: Vec<Bucket<String, i32>>,
    }

    impl Entries for MockEntries {
        type Entry = Bucket<String, i32>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F) where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = MockEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: "key1".to_string(), value: 10 },
        ],
    };

    let index = hash_table::OccupiedEntry::new(&mut entries.entries, 1);
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    // This should panic as there is no entry at index 1
    let _value = occupied_entry.get();
}

