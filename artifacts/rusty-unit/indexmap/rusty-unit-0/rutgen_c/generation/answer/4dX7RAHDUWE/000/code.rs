// Answer 0

#[test]
fn test_insert_changes_value_and_returns_old_value() {
    struct EntryImpl {
        value: i32,
    }

    struct TestEntries {
        entries: Vec<EntryImpl>,
    }

    impl Entries for TestEntries {
        type Entry = EntryImpl;

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
        entries: vec![EntryImpl { value: 10 }] 
    };

    let index = hashbrown::hash_table::OccupiedEntry::from_index(&mut test_entries.entries, 0);

    let mut entry_mut = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index,
        hash_builder: PhantomData,
    };

    let old_value = entry_mut.insert(20);
    
    assert_eq!(old_value, 10);
    assert_eq!(entry_mut.get_mut(), &mut 20);
}

#[test]
fn test_insert_with_default_value() {
    struct EntryImpl {
        value: i32,
    }

    struct TestEntries {
        entries: Vec<EntryImpl>,
    }

    impl Entries for TestEntries {
        type Entry = EntryImpl;

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
        entries: vec![EntryImpl { value: 5 }] 
    };

    let index = hashbrown::hash_table::OccupiedEntry::from_index(&mut test_entries.entries, 0);

    let mut entry_mut = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index,
        hash_builder: PhantomData,
    };

    let old_value = entry_mut.insert(0);
    
    assert_eq!(old_value, 5);
    assert_eq!(entry_mut.get_mut(), &mut 0);
}

