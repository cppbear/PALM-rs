// Answer 0

#[test]
fn test_get_mut_valid_entry() {
    struct TestEntries {
        data: Vec<Bucket<u32, String>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }
        
        fn with_entries<F>(&mut self, f: F) where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: "Value1".to_string() },
            Bucket { hash: HashValue::new(2), key: 2, value: "Value2".to_string() }
        ],
    };

    let index_entry = hash_table::OccupiedEntry::from_index(&mut entries, 1);
    let mut raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    let value_ref = raw_entry.get_mut();
    *value_ref = "UpdatedValue".to_string();

    assert_eq!(raw_entry.get_mut(), &mut "UpdatedValue".to_string());
}

#[test]
fn test_get_mut_out_of_bounds() {
    struct TestEntries {
        data: Vec<Bucket<u32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

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
        data: vec![],
    };

    let index_entry = hash_table::OccupiedEntry::from_index(&mut entries, 0);

    let mut raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_entry,
        hash_builder: PhantomData,
    };

    // Access to the mutable value should be done carefully, as the entry is out of bounds.
    let result = std::panic::catch_unwind(|| {
        raw_entry.get_mut();
    });

    assert!(result.is_err());
}

