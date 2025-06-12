// Answer 0

#[test]
fn test_swap_indices_within_bounds() {
    struct TestEntries {
        entries: Vec<(usize, usize)>, // key-value pairs
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
        entries: vec![(0, 10), (1, 20), (2, 30)],
    };
    
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(1), // We're at index 1
        hash_builder: PhantomData,
    };

    let result = occupied_entry.swap_indices(0); // Swap with index 0
    assert_eq!(entries.as_entries(), &[(1, 20), (0, 10), (2, 30)]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds() {
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
        entries: vec![(0, 10), (1, 20), (2, 30)],
    };
    
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(1), // We're at index 1
        hash_builder: PhantomData,
    };

    occupied_entry.swap_indices(3); // Attempt to swap with an out-of-bounds index
}

