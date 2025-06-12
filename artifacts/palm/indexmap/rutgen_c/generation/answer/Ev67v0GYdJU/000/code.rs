// Answer 0

#[test]
fn test_insert_with_existing_value() {
    struct TestEntries {
        data: Vec<(usize, String)>,
    }
    
    impl Entries for TestEntries {
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
            F: FnOnce(&mut [Self::Entry]) {
                f(&mut self.data);
        }
    }
    
    let mut entries = TestEntries { data: vec![(0, "old_value".to_string())] };
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::from_index(0));

    let old_value = occupied_entry.insert("new_value".to_string());
    assert_eq!(old_value, "old_value");
    assert_eq!(occupied_entry.get(), "new_value");
}

#[test]
fn test_insert_with_empty_value() {
    struct TestEntries {
        data: Vec<(usize, String)>,
    }
    
    impl Entries for TestEntries {
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
            F: FnOnce(&mut [Self::Entry]) {
                f(&mut self.data);
        }
    }
    
    let mut entries = TestEntries { data: vec![(0, "initial_value".to_string())] };
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::from_index(0));

    let old_value = occupied_entry.insert("".to_string());
    assert_eq!(old_value, "initial_value");
    assert_eq!(occupied_entry.get(), "");
}

