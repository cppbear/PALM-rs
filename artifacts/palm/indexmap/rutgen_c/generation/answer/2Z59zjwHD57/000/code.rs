// Answer 0

#[test]
fn test_remove_removes_entry_and_returns_value() {
    struct TestEntries {
        entries: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: vec![(0, "zero".to_string()), (1, "one".to_string()), (2, "two".to_string())] };

    let index = hashbrown::hash_table::OccupiedEntry::new(1, &mut entries.as_entries_mut()[1]);
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    let value = occupied_entry.remove();
    assert_eq!(value, "one");
    assert_eq!(entries.as_entries(), &[(0, "zero".to_string()), (2, "two".to_string())]);
}

#[test]
fn test_remove_on_last_entry() {
    struct TestEntries {
        entries: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: vec![(0, "zero".to_string()), (1, "one".to_string())] };

    let index = hashbrown::hash_table::OccupiedEntry::new(1, &mut entries.as_entries_mut()[1]);
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    let value = occupied_entry.remove();
    assert_eq!(value, "one");
    assert_eq!(entries.as_entries(), &[(0, "zero".to_string())]);
}

#[test]
#[should_panic]
fn test_remove_panics_on_empty_entry() {
    struct TestEntries {
        entries: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: Vec::new() };
    
    let index = hashbrown::hash_table::OccupiedEntry::new(0, &mut entries.as_entries_mut()[0]);
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    occupied_entry.remove();
}

