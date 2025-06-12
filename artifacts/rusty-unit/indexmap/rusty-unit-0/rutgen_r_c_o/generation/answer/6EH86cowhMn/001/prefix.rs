// Answer 0

#[test]
fn test_into_key_with_valid_index() {
    struct TestEntries {
        data: Vec<Bucket<String, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<String, i32>;

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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket { hash: HashValue::default(), key: "key1".to_string(), value: 1 },
            Bucket { hash: HashValue::default(), key: "key2".to_string(), value: 2 },
        ],
    };

    let index = 0;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let key_mut = occupied_entry.into_key();
}

#[test]
fn test_into_key_with_last_index() {
    struct TestEntries {
        data: Vec<Bucket<String, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<String, i32>;

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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket { hash: HashValue::default(), key: "key1".to_string(), value: 1 },
            Bucket { hash: HashValue::default(), key: "key2".to_string(), value: 2 },
        ],
    };

    let index = 1;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let key_mut = occupied_entry.into_key();
}

#[test]
#[should_panic]
fn test_into_key_with_out_of_bounds_index() {
    struct TestEntries {
        data: Vec<Bucket<String, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<String, i32>;

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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket { hash: HashValue::default(), key: "key1".to_string(), value: 1 },
        ],
    };

    let index = 1; // Out of bounds
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let key_mut = occupied_entry.into_key(); // This should panic
}

