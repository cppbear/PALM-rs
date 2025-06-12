// Answer 0

#[test]
fn test_key_mut() {
    struct TestEntries {
        bucket: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.bucket
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &self.bucket
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.bucket
        }
        
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.bucket);
        }
    }

    let mut entries = TestEntries {
        bucket: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: String::from("Entry1") },
            Bucket { hash: HashValue::new(2), key: 2, value: String::from("Entry2") },
        ],
    };

    let index = 0;
    let mut entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    let key_mut_ref = entry_mut.key_mut();
    *key_mut_ref += 10;

    assert_eq!(entries.bucket[index].key, 11);
}

