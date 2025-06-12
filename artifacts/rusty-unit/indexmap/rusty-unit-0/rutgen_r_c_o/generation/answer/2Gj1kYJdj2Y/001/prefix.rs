// Answer 0

#[test]
fn test_get_valid_entry() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            f(&mut self.entries)
        }
    }

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };

    let index = 1;
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(&mut entries.entries, index),
        hash_builder: PhantomData,
    };

    let value = raw_entry.get();
}

#[test]
fn test_get_multiple_valid_entries() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            f(&mut self.entries)
        }
    }

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 4, value: 40 },
            Bucket { hash: HashValue::default(), key: 5, value: 50 },
            Bucket { hash: HashValue::default(), key: 6, value: 60 },
        ],
    };

    for index in 0..entries.entries.len() {
        let raw_entry = RawOccupiedEntryMut {
            entries: &mut entries,
            index: hash_table::OccupiedEntry::new(&mut entries.entries, index),
            hash_builder: PhantomData,
        };

        let value = raw_entry.get();
    }
}

#[test]
#[should_panic]
fn test_get_empty_entries() {
    struct EmptyEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for EmptyEntries {
        type Entry = Bucket<i32, i32>;

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
            f(&mut self.entries)
        }
    }

    let mut entries = EmptyEntries {
        entries: Vec::new(),
    };

    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(&mut entries.entries, 0), // Out of bounds
        hash_builder: PhantomData,
    };

    let _ = raw_entry.get();
}

