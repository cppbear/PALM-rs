// Answer 0

#[test]
fn test_remove_entry() {
    struct TestEntries {
        entries: Vec<(usize, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (usize, String);
        
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

    let mut test_entries = TestEntries { entries: vec![(0, "zero".into()), (1, "one".into()), (2, "two".into())] };
    let index = test_entries.as_entries_mut().len() - 1; // Take the last entry for testing removal
    let occupied_entry = OccupiedEntry::new(&mut test_entries, hashbrown::hash_table::OccupiedEntry { index });

    let (key, value) = occupied_entry.remove_entry();

    assert_eq!(key, 2);
    assert_eq!(value, "two");
    assert_eq!(test_entries.entries.len(), 2);
    assert_eq!(test_entries.entries.last(), Some(&(1, "one".into())));
}

