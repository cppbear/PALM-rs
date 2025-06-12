// Answer 0

#[test]
fn test_occupied_entry_debug_valid() {
    struct TestEntries {
        entries: Vec<(String, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (String, String);
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
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ],
    };

    let index = 0;
    let occupied_entry = OccupiedEntry::new(&mut test_entries, hash_table::OccupiedEntry::new(index));

    let _ = fmt::Debug::fmt(&occupied_entry, &mut fmt::Formatter::new());
}

#[test]
#[should_panic]
fn test_occupied_entry_debug_invalid_index() {
    struct TestEntries {
        entries: Vec<(String, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (String, String);
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
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ],
    };

    let index = 2; // Invalid index
    let occupied_entry = OccupiedEntry::new(&mut test_entries, hash_table::OccupiedEntry::new(index));

    let _ = fmt::Debug::fmt(&occupied_entry, &mut fmt::Formatter::new());
}

#[test]
fn test_occupied_entry_debug_non_empty() {
    struct TestEntries {
        entries: Vec<(String, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (String, String);
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
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ],
    };

    let index = 1;
    let occupied_entry = OccupiedEntry::new(&mut test_entries, hash_table::OccupiedEntry::new(index));

    let _ = fmt::Debug::fmt(&occupied_entry, &mut fmt::Formatter::new());
}

