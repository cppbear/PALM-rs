// Answer 0

#[test]
fn test_remove_entry() {
    struct DummyEntries {
        data: Vec<(usize, String)>,
    }

    impl Entries for DummyEntries {
        type Entry = (usize, String);

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data)
        }
    }

    let mut entries = DummyEntries { data: vec![] };
    entries.data.push((0, "zero".to_string()));
    entries.data.push((1, "one".to_string()));

    let index = hash_table::OccupiedEntry::new(&mut entries.data, 1).unwrap();
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    let (key, value) = occupied_entry.remove_entry();
    
    assert_eq!(key, 1);
    assert_eq!(value, "one".to_string());
    assert_eq!(entries.as_entries().len(), 1);
}

#[test]
#[should_panic]
fn test_remove_entry_empty() {
    struct DummyEntries {
        data: Vec<(usize, String)>,
    }

    impl Entries for DummyEntries {
        type Entry = (usize, String);

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data)
        }
    }

    let mut entries = DummyEntries { data: vec![] };
    
    // Attempting to remove from empty entries
    let index = hash_table::OccupiedEntry::new(&mut entries.data, 0).unwrap(); 
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    occupied_entry.remove_entry(); // This should panic
}

