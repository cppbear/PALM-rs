// Answer 0

#[test]
fn test_into_key_value_mut() {
    struct TestEntries<K, V> {
        inner: Vec<Bucket<K, V>>,
    }
    
    impl<K, V> Entries for TestEntries<K, V> {
        type Entry = Bucket<K, V>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.inner
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &self.inner
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.inner
        }
        
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.inner);
        }
    }

    let mut entries = TestEntries {
        inner: vec![
            Bucket { hash: HashValue::default(), key: "key1", value: 10 },
            Bucket { hash: HashValue::default(), key: "key2", value: 20 },
        ],
    };

    let index_entry = hash_table::OccupiedEntry::from_index(0);
    
    let mut raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    let (key, value) = raw_entry.into_key_value_mut();
    
    assert_eq!(*key, "key1");
    assert_eq!(*value, 10);
    
    *value = 30; // Modify value
    assert_eq!(entries.as_entries()[0].value_ref(), &30);

    *key = "modified_key"; // Modify key
    assert_eq!(entries.as_entries()[0].key_ref(), &"modified_key");
}

