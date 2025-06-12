// Answer 0

#[test]
fn test_fmt_with_valid_key_value() {
    struct TestEntry {
        key: String,
        value: i32,
    }

    struct TestEntries {
        entries: Vec<TestEntry>,
    }

    impl Entries for TestEntries {
        type Entry = TestEntry;

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

    let mut entries = TestEntries {
        entries: vec![TestEntry {
            key: "TestKey".to_string(),
            value: 42,
        }],
    };
    let index = hashbrown::hash_table::OccupiedEntry::from_entry_index(&mut entries, 0).unwrap();
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };
    
    let result = format!("{:?}", raw_entry);
    assert!(result.contains("key: \"TestKey\""));
    assert!(result.contains("value: 42"));
}

#[test]
#[should_panic]
fn test_fmt_with_empty_entries() {
    struct TestEntry {
        key: String,
        value: i32,
    }

    struct TestEntries {
        entries: Vec<TestEntry>,
    }

    impl Entries for TestEntries {
        type Entry = TestEntry;

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

    let mut entries = TestEntries { entries: vec![] };
    // Attempting to access a key/value should panic
    let index = hashbrown::hash_table::OccupiedEntry::from_entry_index(&mut entries, 0).unwrap();
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };
    
    let _ = format!("{:?}", raw_entry);
}

