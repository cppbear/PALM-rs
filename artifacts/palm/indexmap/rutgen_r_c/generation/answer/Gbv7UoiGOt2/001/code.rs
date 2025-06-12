// Answer 0

#[test]
fn test_into_key_value_mut_valid_entry() {
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
            Bucket { hash: HashValue::new(0), key: 1, value: String::from("a") },
            Bucket { hash: HashValue::new(1), key: 2, value: String::from("b") },
        ],
    };

    let index = 0; // Valid index
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let (key, value) = raw_entry.into_key_value_mut();
    *key = 3; // Modify key
    *value = String::from("modified"); // Modify value

    assert_eq!(entries.entries[0].key, 3);
    assert_eq!(entries.entries[0].value, "modified");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_into_key_value_mut_invalid_entry() {
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
            Bucket { hash: HashValue::new(0), key: 1, value: String::from("a") },
        ],
    };

    let index = 1; // Invalid index
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let _ = raw_entry.into_key_value_mut(); // This should panic
}

