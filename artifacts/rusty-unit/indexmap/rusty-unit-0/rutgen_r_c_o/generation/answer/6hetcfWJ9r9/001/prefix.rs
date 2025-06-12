// Answer 0

#[test]
fn test_insert_key_minimum_value() {
    struct TestEntries {
        entries: Vec<(u32, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (u32, String);
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
            f(&mut self.entries)
        }
    }

    let mut test_entries = TestEntries { entries: vec![(1, "value1".to_string())] };
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied(0), // mimic occupied entry
        hash_builder: PhantomData,
    };

    let old_key = entry.insert_key(0);
}

#[test]
fn test_insert_key_maximum_value() {
    struct TestEntries {
        entries: Vec<(u32, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (u32, String);
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
            f(&mut self.entries)
        }
    }

    let mut test_entries = TestEntries { entries: vec![(u32::MAX, "value_max".to_string())] };
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied(0), // mimic occupied entry
        hash_builder: PhantomData,
    };

    let old_key = entry.insert_key(u32::MAX);
}

#[test]
fn test_insert_key_unique_key() {
    struct TestEntries {
        entries: Vec<(u32, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (u32, String);
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
            f(&mut self.entries)
        }
    }

    let mut test_entries = TestEntries { entries: vec![(2, "value2".to_string())] };
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied(0), // mimic occupied entry
        hash_builder: PhantomData,
    };

    let old_key = entry.insert_key(3);
}

#[test]
#[should_panic]
fn test_insert_key_duplicate_key() {
    struct TestEntries {
        entries: Vec<(u32, String)>,
    }

    impl Entries for TestEntries {
        type Entry = (u32, String);
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
            f(&mut self.entries)
        }
    }

    let mut test_entries = TestEntries { entries: vec![(4, "value4".to_string()), (5, "value5".to_string())] };
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied(0), // mimic occupied entry
        hash_builder: PhantomData,
    };

    entry.insert_key(4); // Inserting a duplicate key should panic.
}

