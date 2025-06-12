// Answer 0

#[test]
fn test_get_key_value_mut_valid_case() {
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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.buckets);
        }
    }

    let mut entries = TestEntries { 
        buckets: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "First".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "Second".to_string() },
        ],
    };

    let index = 0; // Testing the entry at index 0
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(&mut entries.buckets, index),
        hash_builder: PhantomData,
    };

    let (key, value) = occupied_entry.get_key_value_mut();
    assert_eq!(*key, 1);
    assert_eq!(*value, "First");
}

#[should_panic]
#[test]
fn test_get_key_value_mut_invalid_case() {
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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.buckets);
        }
    }

    let mut entries = TestEntries { 
        buckets: vec![],
    };

    let index = 0; // Testing an invalid index
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(&mut entries.buckets, index),
        hash_builder: PhantomData,
    };

    // This should panic due to the empty bucket list
    occupied_entry.get_key_value_mut();
}

