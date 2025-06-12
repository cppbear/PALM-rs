// Answer 0

#[test]
fn test_into_key_value_mut_valid() {
    struct TestEntries {
        entries: Vec<Bucket<u32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

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
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "value1".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "value2".to_string() },
        ],
    };

    let index = 0;
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied { key: &1, index },
        hash_builder: PhantomData,
    };

    let (key_mut, value_mut) = entry.into_key_value_mut();
}

#[test]
#[should_panic]
fn test_into_key_value_mut_out_of_bounds_index() {
    struct TestEntries {
        entries: Vec<Bucket<u32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

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
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "value1".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "value2".to_string() },
        ],
    };

    let index = 2; // Out of bounds index
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied { key: &2, index },
        hash_builder: PhantomData,
    };

    let _ = entry.into_key_value_mut(); // This should panic
}

#[test]
fn test_into_key_value_mut_correct_values() {
    struct TestEntries {
        entries: Vec<Bucket<u32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

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
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "value1".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "value2".to_string() },
        ],
    };

    let index = 1;
    let mut entry = RawOccupiedEntryMut {
        entries: &mut test_entries,
        index: hash_table::OccupiedEntry::Occupied { key: &2, index },
        hash_builder: PhantomData,
    };

    let (key_mut, value_mut) = entry.into_key_value_mut();
    *value_mut = "new_value".to_string(); // Modifying the value through the mutable reference
}

