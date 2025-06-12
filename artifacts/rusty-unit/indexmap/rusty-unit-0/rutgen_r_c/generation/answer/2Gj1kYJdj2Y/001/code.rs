// Answer 0

#[test]
fn test_get_valid_entry() {
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
            Bucket { hash: HashValue(1), key: 1, value: "value1".to_string() },
            Bucket { hash: HashValue(2), key: 2, value: "value2".to_string() },
        ],
    };
    
    let index = 0;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    assert_eq!(occupied_entry.get(), "value1");
}

#[test]
#[should_panic]
fn test_get_out_of_bounds() {
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
            Bucket { hash: HashValue(1), key: 1, value: "value1".to_string() },
        ],
    };
    
    let index = 1; // out of bounds
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    occupied_entry.get();
}

