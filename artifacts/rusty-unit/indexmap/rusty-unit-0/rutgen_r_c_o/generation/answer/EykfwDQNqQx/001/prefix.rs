// Answer 0

#[test]
fn test_into_mut_valid() {
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
            Bucket { hash: HashValue::from(1), key: "key1".to_string(), value: 10 },
            Bucket { hash: HashValue::from(2), key: "key2".to_string(), value: 20 },
        ]
    };

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(0),
        hash_builder: PhantomData,
    };

    let value_mut = occupied_entry.into_mut();
}

#[test]
fn test_into_mut_multiple_entries() {
    struct TestEntries {
        data: Vec<Bucket<i32, f64>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, f64>;

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
            Bucket { hash: HashValue::from(3), key: 1, value: 10.5 },
            Bucket { hash: HashValue::from(4), key: 2, value: 20.5 },
            Bucket { hash: HashValue::from(5), key: 3, value: 30.5 },
        ]
    };

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(1),
        hash_builder: PhantomData,
    };

    let value_mut = occupied_entry.into_mut();
}

#[test]
#[should_panic]
fn test_into_mut_index_out_of_bounds() {
    struct TestEntries {
        data: Vec<Bucket<u32, char>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, char>;

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
            Bucket { hash: HashValue::from(6), key: 5, value: 'A' },
        ]
    };

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(2), // Invalid index
        hash_builder: PhantomData,
    };

    let _value_mut = occupied_entry.into_mut();
}

#[test]
fn test_into_mut_single_entry() {
    struct TestEntries {
        data: Vec<Bucket<bool, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<bool, String>;

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
            Bucket { hash: HashValue::from(7), key: true, value: "Single".to_string() },
        ]
    };

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(0),
        hash_builder: PhantomData,
    };

    let value_mut = occupied_entry.into_mut();
}

