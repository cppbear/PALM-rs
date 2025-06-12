// Answer 0

#[test]
fn test_remove_with_valid_entry() {
    struct MockEntries {
        entries: Vec<(usize, isize)>,
    }

    impl Entries for MockEntries {
        type Entry = (usize, isize);

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

    let mut entries = MockEntries { entries: vec![(0, 10), (1, 20), (2, 30)] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };

    let value = occupied_entry.remove();
    assert_eq!(value, 10);
    assert_eq!(entries.as_entries(), &[(0, 10), (2, 30)]);
}

#[test]
#[should_panic]
fn test_remove_on_empty_entry() {
    struct MockEntries {
        entries: Vec<(usize, isize)>,
    }

    impl Entries for MockEntries {
        type Entry = (usize, isize);

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

    let mut entries = MockEntries { entries: vec![] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };

    occupied_entry.remove();
}

#[test]
fn test_remove_on_single_element_entry() {
    struct MockEntries {
        entries: Vec<(usize, isize)>,
    }

    impl Entries for MockEntries {
        type Entry = (usize, isize);

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

    let mut entries = MockEntries { entries: vec![(0, 10)] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };

    let value = occupied_entry.remove();
    assert_eq!(value, 10);
    assert!(entries.as_entries().is_empty());
}

#[test]
fn test_remove_and_check_order() {
    struct MockEntries {
        entries: Vec<(usize, isize)>,
    }

    impl Entries for MockEntries {
        type Entry = (usize, isize);

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

    let mut entries = MockEntries { entries: vec![(0, 10), (1, 20), (2, 30)] };
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(1),
        hash_builder: PhantomData,
    };

    let value = occupied_entry.remove();
    assert_eq!(value, 20);
    assert_eq!(entries.as_entries(), &[(0, 10), (2, 30)]);
}

