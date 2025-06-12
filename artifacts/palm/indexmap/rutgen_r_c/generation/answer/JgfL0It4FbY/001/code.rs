// Answer 0

#[test]
fn test_shift_remove_entry() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, usize);
        
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

    let mut entries = TestEntries { entries: vec![(0, 1), (1, 2), (2, 3), (3, 4)] };
    
    let index = hashbrown::hash_table::OccupiedEntry::new(1); // Assuming 1 is the index to remove
    let mut occupied_entry = OccupiedEntry::new(&mut entries, index);
    
    let (key, value) = occupied_entry.shift_remove_entry();
    
    assert_eq!(key, 1);
    assert_eq!(value, 2);
    assert_eq!(entries.entries.len(), 3);
    assert_eq!(entries.entries, vec![(0, 1), (2, 3), (3, 4)]);
}

#[test]
#[should_panic]
fn test_shift_remove_entry_out_of_bounds() {
    struct TestEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, usize);
        
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

    let mut entries = TestEntries { entries: vec![(0, 1), (1, 2)] };
    
    let index = hashbrown::hash_table::OccupiedEntry::new(2); // Invalid index
    let mut occupied_entry = OccupiedEntry::new(&mut entries, index);
    
    occupied_entry.shift_remove_entry();
}

