// Answer 0

#[test]
fn test_get_value() {
    struct TestEntries<K, V> {
        data: Vec<Bucket<K, V>>,
    }

    impl<K, V> Entries for TestEntries<K, V> {
        type Entry = Bucket<K, V>;

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

    let mut entries = TestEntries { data: vec![
        Bucket { hash: HashValue::new(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue::new(2), key: "key2", value: "value2" }
    ]};

    let index_entry = hash_table::OccupiedEntry::from_index(0);

    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    assert_eq!(raw_entry.get(), &"value1");
}

#[test]
fn test_get_nonexistent_value() {
    struct TestEntries<K, V> {
        data: Vec<Bucket<K, V>>,
    }

    impl<K, V> Entries for TestEntries<K, V> {
        type Entry = Bucket<K, V>;

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

    let mut entries = TestEntries { data: vec![] };

    let index_entry = hash_table::OccupiedEntry::from_index(0); // invalid index

    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    // this should panic due to attempted access of an invalid index
    let result = std::panic::catch_unwind(|| {
        raw_entry.get();
    });
    
    assert!(result.is_err());
}

