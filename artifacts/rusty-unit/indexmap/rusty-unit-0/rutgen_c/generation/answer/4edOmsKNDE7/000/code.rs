// Answer 0

#[test]
fn test_shift_remove() {
    struct TestEntries {
        data: Vec<(i32, i32)>,
    }

    impl Entries for TestEntries {
        type Entry = (i32, i32);

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

    let mut entries = TestEntries { data: vec![(1, 10), (2, 20), (3, 30)] };
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(),
        hash_builder: PhantomData,
    };

    // Assuming the entries index is at position 1
    let removed_value = occupied_entry.shift_remove();

    assert_eq!(removed_value, 20);
    assert_eq!(entries.as_entries(), &[(1, 10), (3, 30)]);
}

#[test]
fn test_shift_remove_boundary_conditions() {
    struct TestEntries {
        data: Vec<(i32, i32)>,
    }

    impl Entries for TestEntries {
        type Entry = (i32, i32);

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

    let mut entries = TestEntries { data: vec![(1, 10)] };
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(),
        hash_builder: PhantomData,
    };

    // Remove the only entry
    let removed_value = occupied_entry.shift_remove();

    assert_eq!(removed_value, 10);
    assert!(entries.as_entries().is_empty());
}

#[test]
#[should_panic]
fn test_shift_remove_empty_entries() {
    struct TestEntries {
        data: Vec<(i32, i32)>,
    }

    impl Entries for TestEntries {
        type Entry = (i32, i32);

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

    let mut entries = TestEntries { data: Vec::new() };
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(),
        hash_builder: PhantomData,
    };

    // Attempt to shift remove from an empty entry
    occupied_entry.shift_remove();
}

