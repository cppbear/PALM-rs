// Answer 0

#[test]
fn test_index_valid_entry() {
    struct TestEntries {
        data: Vec<(usize, usize)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, usize);

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
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(1, 10), (2, 20), (3, 30)] };
    let index = 1; // This index is within bounds

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(index),
        hash_builder: PhantomData,
    };

    assert_eq!(occupied_entry.index(), index);
}

#[test]
#[should_panic]
fn test_index_invalid_entry() {
    struct TestEntries {
        data: Vec<(usize, usize)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, usize);

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
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(1, 10)] };
    let index = 10; // This index is out of bounds

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(index),
        hash_builder: PhantomData,
    };

    let _ = occupied_entry.index(); // This should panic
}

