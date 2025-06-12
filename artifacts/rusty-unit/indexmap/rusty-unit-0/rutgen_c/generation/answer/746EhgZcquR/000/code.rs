// Answer 0

#[test]
fn test_swap_remove() {
    struct TestEntries {
        values: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.values
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.values
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.values
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.values);
        }
    }

    let mut entries = TestEntries { values: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())] };

    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(0), // Assume occupied in index 0
        hash_builder: PhantomData,
    };

    let removed_value = occupied_entry.swap_remove();
    assert_eq!(removed_value, "one".to_string());
    assert_eq!(entries.as_entries(), &[(3, "three".to_string()), (2, "two".to_string())]);
}

#[test]
#[should_panic]
fn test_swap_remove_empty() {
    struct TestEntries {
        values: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.values
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.values
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.values
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.values);
        }
    }

    let mut entries = TestEntries { values: vec![] };

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(0), // This should panic
        hash_builder: PhantomData,
    };

    occupied_entry.swap_remove();
}

