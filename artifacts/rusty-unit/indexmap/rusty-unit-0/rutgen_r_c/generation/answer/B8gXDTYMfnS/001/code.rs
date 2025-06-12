// Answer 0

#[test]
fn test_key_mut_valid_access() {
    struct TestEntries<'a> {
        entries: Vec<Bucket<i32, String>>,
    }

    impl<'a> Entries for TestEntries<'a> {
        type Entry = Bucket<i32, String>;

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

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "value1".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "value2".to_string() },
        ],
    };

    let occupied_entry_index = 0; // The index we are testing
    let mut raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(occupied_entry_index),
        hash_builder: PhantomData,
    };

    let key_mut = raw_entry.key_mut();
    assert_eq!(*key_mut, 1);
    *key_mut = 3;
    assert_eq!(entries.entries[occupied_entry_index].key, 3);
}

#[should_panic]
fn test_key_mut_access_out_of_bounds() {
    struct TestEntries<'a> {
        entries: Vec<Bucket<i32, String>>,
    }

    impl<'a> Entries for TestEntries<'a> {
        type Entry = Bucket<i32, String>;

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

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "value1".to_string() },
        ],
    };

    let occupied_entry_index = 1; // Invalid index
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(occupied_entry_index),
        hash_builder: PhantomData,
    };

    let _ = raw_entry.key_mut(); // This should panic
}

