// Answer 0

#[test]
fn test_get_mut_valid_index() {
    struct TestEntries {
        data: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };
    
    let index = 1;
    let mut entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from(index),
        hash_builder: PhantomData,
    };

    let value_mut = entry_mut.get_mut();
}

#[test]
#[should_panic]
fn test_get_mut_index_out_of_bounds() {
    struct TestEntries {
        data: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
        ],
    };

    let index = 1; // Out of bounds index
    let mut entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from(index),
        hash_builder: PhantomData,
    };
    
    let value_mut = entry_mut.get_mut();
}

#[test]
fn test_get_mut_multiple_entries() {
    struct TestEntries {
        data: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            Bucket { hash: HashValue::default(), key: 1, value: 100 },
            Bucket { hash: HashValue::default(), key: 2, value: 200 },
            Bucket { hash: HashValue::default(), key: 3, value: 300 },
        ],
    };

    for index in 0..entries.data.len() {
        let mut entry_mut = RawOccupiedEntryMut {
            entries: &mut entries,
            index: hash_table::OccupiedEntry::from(index),
            hash_builder: PhantomData,
        };

        let value_mut = entry_mut.get_mut();
    }
}

