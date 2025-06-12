// Answer 0

#[test]
fn test_move_index_within_bounds() {
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

    let mut entries = TestEntries {
        entries: vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)],
    };

    let index_entry = hash_table::OccupiedEntry::from_index(1);
    let occupied_entry = OccupiedEntry::new(&mut entries, index_entry);

    occupied_entry.move_index(3);

    assert_eq!(entries.entries, vec![(0, 0), (2, 2), (3, 3), (1, 1), (4, 4)]);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds() {
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

    let mut entries = TestEntries {
        entries: vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)],
    };

    let index_entry = hash_table::OccupiedEntry::from_index(1);
    let occupied_entry = OccupiedEntry::new(&mut entries, index_entry);

    occupied_entry.move_index(5); // Out of bounds
}

