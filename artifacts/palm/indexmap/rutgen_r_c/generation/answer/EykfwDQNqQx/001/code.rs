// Answer 0

#[test]
fn test_into_mut_valid_entry() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
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
            Bucket { hash: HashValue::default(), key: 1, value: "first".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "second".to_string() },
        ],
    };

    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(1), // Assume valid index
        hash_builder: PhantomData,
    };

    let value_ref = occupied_entry.into_mut();
    assert_eq!(*value_ref, "second".to_string());
    *value_ref = "modified".to_string();
    assert_eq!(entries.entries[1].value, "modified".to_string());
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_into_mut_invalid_entry() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
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
            Bucket { hash: HashValue::default(), key: 1, value: "first".to_string() },
        ],
    };

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(1), // Invalid index
        hash_builder: PhantomData,
    };

    let _value_ref = occupied_entry.into_mut(); // This should panic
}

