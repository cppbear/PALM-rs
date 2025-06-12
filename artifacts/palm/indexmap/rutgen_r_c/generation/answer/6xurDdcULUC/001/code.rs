// Answer 0

#[test]
fn test_into_muts_valid() {
    use crate::HashValue;
    use hashbrown::hash_table;

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

    let mut entries = TestEntries { data: vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a".to_string() },
        Bucket { hash: HashValue::default(), key: 2, value: "b".to_string() },
    ]};

    let entry_index = hash_table::OccupiedEntry::from_raw(0); // Assume a valid index here
    let occupied_entry = OccupiedEntry::new(&mut entries, entry_index);
    
    let (key_mut, value_mut) = occupied_entry.into_muts();
    *key_mut = 3;
    *value_mut = "c".to_string();

    assert_eq!(entries.data[0].key, 3);
    assert_eq!(entries.data[0].value, "c");
}

#[should_panic]
fn test_into_muts_out_of_bounds() {
    use crate::HashValue;
    use hashbrown::hash_table;

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

    let mut entries = TestEntries { data: vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a".to_string() },
    ]};

    let entry_index = hash_table::OccupiedEntry::from_raw(1); // Invalid index
    let occupied_entry = OccupiedEntry::new(&mut entries, entry_index);
    
    let _ = occupied_entry.into_muts(); // This should panic
}

