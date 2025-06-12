// Answer 0

#[test]
fn test_index_valid() {
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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut test_entries = TestEntries { data: vec![(1, 10), (2, 20)] };
    let occupied_index = hashbrown::hash_table::OccupiedEntry::from_index(1);

    let occupied_entry = OccupiedEntry::new(&mut test_entries, occupied_index);
    assert_eq!(occupied_entry.index(), 1);
}

#[test]
fn test_index_empty() {
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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut test_entries = TestEntries { data: vec![] };
    let occupied_index = hashbrown::hash_table::OccupiedEntry::from_index(0);

    let occupied_entry = OccupiedEntry::new(&mut test_entries, occupied_index);
    assert_eq!(occupied_entry.index(), 0);
}

