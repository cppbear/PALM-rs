// Answer 0

#[test]
fn test_insert_key_updates_key_and_returns_old_key() {
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

    let mut test_entries = TestEntries {
        entries: vec![(0, String::from("old_key"))],
    };

    let mut occupied_entry_mut = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hashbrown::hash_table::OccupiedEntry::from_index(0),
        hash_builder: PhantomData::<()>,
    };

    let old_key = occupied_entry_mut.insert_key(String::from("new_key"));

    assert_eq!(old_key, "old_key");
    assert_eq!(occupied_entry_mut.key(), "new_key");
}

#[test]
fn test_insert_key_changes_key_and_verifies_entry() {
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

    let mut test_entries = TestEntries {
        entries: vec![(1, String::from("initial_key"))],
    };

    let mut occupied_entry_mut = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hashbrown::hash_table::OccupiedEntry::from_index(0),
        hash_builder: PhantomData::<()>,
    };

    let old_key = occupied_entry_mut.insert_key(String::from("updated_key"));

    assert_eq!(old_key, "initial_key");
    assert_eq!(occupied_entry_mut.key(), "updated_key");
}

