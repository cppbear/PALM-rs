// Answer 0

#[test]
fn test_swap_indices_within_bounds() {
    struct TestEntries {
        data: Vec<usize>,
    }
    
    impl Entries for TestEntries {
        type Entry = usize;

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

    let mut entries = TestEntries { data: vec![0, 1, 2, 3, 4] };
    let index = 2;
    let other_index = 4;

    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(index),
        hash_builder: PhantomData,
    };
    
    raw_entry.swap_indices(other_index);
    
    assert_eq!(entries.data, vec![0, 1, 4, 3, 2]);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_swap_indices_out_of_bounds() {
    struct TestEntries {
        data: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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

    let mut entries = TestEntries { data: vec![0, 1, 2, 3, 4] };
    let index = 1;
    let out_of_bounds_index = 5;

    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(index),
        hash_builder: PhantomData,
    };

    raw_entry.swap_indices(out_of_bounds_index);
} 

#[test]
fn test_swap_indices_same_index() {
    struct TestEntries {
        data: Vec<usize>,
    }

    impl Entries for TestEntries {
        type Entry = usize;

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

    let mut entries = TestEntries { data: vec![10, 20, 30, 40] };
    let index = 1;
    let same_index = 1;

    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(index),
        hash_builder: PhantomData,
    };

    raw_entry.swap_indices(same_index);
    
    assert_eq!(entries.data, vec![10, 20, 30, 40]);
}

