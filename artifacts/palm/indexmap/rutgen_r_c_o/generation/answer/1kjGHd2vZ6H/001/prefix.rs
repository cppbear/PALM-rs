// Answer 0

#[test]
fn test_get_key_value_valid_entry() {
    struct TestEntries {
        data: Vec<Bucket<u32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket {
                hash: HashValue::from(1),
                key: 1,
                value: "Value1".to_string(),
            },
            Bucket {
                hash: HashValue::from(2),
                key: 2,
                value: "Value2".to_string(),
            },
        ],
    };

    let index = 0;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_entry_index(index, 0),
        hash_builder: PhantomData,
    };

    let result = occupied_entry.get_key_value();
}

#[test]
fn test_get_key_value_multiple_entries() {
    struct TestEntries {
        data: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket {
                hash: HashValue::from(1),
                key: 10,
                value: "Ten".to_string(),
            },
            Bucket {
                hash: HashValue::from(2),
                key: 20,
                value: "Twenty".to_string(),
            },
            Bucket {
                hash: HashValue::from(3),
                key: 30,
                value: "Thirty".to_string(),
            },
        ],
    };

    for index in 0..entries.data.len() {
        let occupied_entry = RawOccupiedEntryMut {
            entries: &mut entries,
            index: hash_table::OccupiedEntry::from_entry_index(index, 0),
            hash_builder: PhantomData,
        };

        let result = occupied_entry.get_key_value();
    }
}

#[test]
#[should_panic]
fn test_get_key_value_out_of_bounds() {
    struct TestEntries {
        data: Vec<Bucket<u64, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u64, String>;

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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket {
                hash: HashValue::from(1),
                key: 100,
                value: "One Hundred".to_string(),
            },
        ],
    };

    let index = 1; // Out of bounds index
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_entry_index(index, 0),
        hash_builder: PhantomData,
    };

    let result = occupied_entry.get_key_value();
}

