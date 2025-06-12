// Answer 0

#[test]
fn test_remove_with_valid_entry() {
    struct MockEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for MockEntries {
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }
    
    let mut mock_entries = MockEntries { entries: vec![(0, 10), (1, 20), (2, 30)] };
    let occupied_entry = OccupiedEntry::new(&mut mock_entries, hash_table::OccupiedEntry::from_index(&mut mock_entries.entries, 1).unwrap());

    let removed_value = occupied_entry.remove();

    assert_eq!(removed_value, 20);
}

#[test]
#[should_panic]
fn test_remove_with_empty_map() {
    struct EmptyMockEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for EmptyMockEntries {
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut empty_mock_entries = EmptyMockEntries { entries: vec![] };
    let occupied_entry = OccupiedEntry::new(&mut empty_mock_entries, hash_table::OccupiedEntry::from_index(&mut empty_mock_entries.entries, 0).unwrap());

    occupied_entry.remove();
}

#[test]
fn test_remove_edge_case() {
    struct EdgeCaseEntries {
        entries: Vec<(usize, usize)>,
    }

    impl Entries for EdgeCaseEntries {
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut edge_case_entries = EdgeCaseEntries { entries: vec![(0, 5)] };
    let occupied_entry = OccupiedEntry::new(&mut edge_case_entries, hash_table::OccupiedEntry::from_index(&mut edge_case_entries.entries, 0).unwrap());

    let removed_value = occupied_entry.remove();

    assert_eq!(removed_value, 5);
    assert!(edge_case_entries.entries.is_empty());
}

