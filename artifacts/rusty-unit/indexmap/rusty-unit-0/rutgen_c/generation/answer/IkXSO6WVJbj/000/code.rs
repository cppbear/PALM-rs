// Answer 0

#[test]
fn test_key_returns_correct_key() {
    struct TestEntries {
        data: Vec<Bucket<i32, &str>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, &str>;

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
            Bucket { hash: HashValue::default(), key: 1, value: "one" },
            Bucket { hash: HashValue::default(), key: 2, value: "two" },
        ],
    };

    let index = hash_table::OccupiedEntry::from_index(0);
    let mut occupied_entry = OccupiedEntry::new(&mut entries, index);
    assert_eq!(*occupied_entry.key(), 1);
}

#[test]
fn test_key_with_different_key() {
    struct TestEntries {
        data: Vec<Bucket<String, &str>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<String, &str>;

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
            Bucket { hash: HashValue::default(), key: "key1".to_string(), value: "value1" },
            Bucket { hash: HashValue::default(), key: "key2".to_string(), value: "value2" },
        ],
    };

    let index = hash_table::OccupiedEntry::from_index(1);
    let mut occupied_entry = OccupiedEntry::new(&mut entries, index);
    assert_eq!(occupied_entry.key(), &"key2".to_string());
}

