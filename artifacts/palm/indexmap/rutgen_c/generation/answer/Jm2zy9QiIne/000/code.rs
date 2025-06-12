// Answer 0

#[test]
fn test_get_mut_valid_entry() {
    struct MockEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for MockEntries {
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

    let mut mock_entries = MockEntries {
        entries: vec![
            Bucket { hash: HashValue::from(1), key: 1, value: "value1".to_string() },
            Bucket { hash: HashValue::from(2), key: 2, value: "value2".to_string() },
        ],
    };

    let index = hashbrown::hash_table::OccupiedEntry::from(&mut mock_entries.as_entries_mut(), 1);
    let mut occupied_entry = OccupiedEntry::new(&mut mock_entries, index);
    
    let value_mut = occupied_entry.get_mut();
    *value_mut = "updated_value".to_string();

    assert_eq!(occupied_entry.get(), "updated_value");
}

#[test]
#[should_panic]
fn test_get_mut_out_of_bounds() {
    struct MockEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for MockEntries {
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

    let mut mock_entries = MockEntries {
        entries: vec![
            Bucket { hash: HashValue::from(1), key: 1, value: "value1".to_string() },
        ],
    };

    let index = hashbrown::hash_table::OccupiedEntry::from(&mut mock_entries.as_entries_mut(), 5);
    let mut occupied_entry = OccupiedEntry::new(&mut mock_entries, index);
    occupied_entry.get_mut();  // This should panic
}

