// Answer 0

#[test]
fn test_occupied_entry_new() {
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

    let mut entries = TestEntries { data: vec![1, 2, 3] };

    // Simulate a valid occupied entry index
    let index = hash_table::OccupiedEntry::new(&mut entries.as_entries_mut()[..], 1).unwrap();

    let occupied_entry = OccupiedEntry::new(&mut entries, index);
    assert_eq!(occupied_entry.index, index);
}

#[test]
#[should_panic]
fn test_occupied_entry_new_with_invalid_index() {
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

    let mut entries = TestEntries { data: vec![1, 2, 3] };

    // Attempt to create an occupied entry with an out-of-bounds index
    let invalid_index = 5; // Out of bounds
    let index = hash_table::OccupiedEntry::new(&mut entries.as_entries_mut()[..], invalid_index).unwrap();

    let _ = OccupiedEntry::new(&mut entries, index);
}

