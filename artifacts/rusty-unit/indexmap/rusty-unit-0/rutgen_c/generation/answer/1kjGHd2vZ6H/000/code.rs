// Answer 0

#[test]
fn test_get_key_value() {
    struct TestEntries {
        buckets: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.buckets
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.buckets
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.buckets
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.buckets);
        }
    }

    let mut entries = TestEntries {
        buckets: vec![
            Bucket {
                hash: HashValue::default(),
                key: 1,
                value: "one".to_string(),
            },
            Bucket {
                hash: HashValue::default(),
                key: 2,
                value: "two".to_string(),
            },
        ],
    };

    let index = 0; // Assume we are accessing the first entry

    let raw_occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let (key, value) = raw_occupied_entry.get_key_value();

    assert_eq!(*key, 1);
    assert_eq!(*value, "one");
}

#[test]
fn test_get_key_value_boundary() {
    struct TestEntries {
        buckets: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.buckets
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.buckets
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.buckets
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.buckets);
        }
    }

    let mut entries = TestEntries {
        buckets: vec![Bucket {
            hash: HashValue::default(),
            key: 1,
            value: "one".to_string(),
        }],
    };

    let index = 0; // Only one entry present

    let raw_occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let (key, value) = raw_occupied_entry.get_key_value();

    assert_eq!(*key, 1);
    assert_eq!(*value, "one");
}

