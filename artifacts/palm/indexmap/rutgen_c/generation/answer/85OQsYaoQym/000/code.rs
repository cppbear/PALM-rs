// Answer 0

#[test]
fn test_insert_full_vacant_entry() {
    struct TestEntries;
    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            Vec::new()
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }

        fn with_entries<F>(&mut self, _f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {}
    }

    let mut index_map = IndexMapCore::<u32, String>::new();
    let hash_value = HashValue(42);
    let (index, old_value) = index_map.insert_full(hash_value, 1, "test".to_string());
    
    assert_eq!(index, 0);
    assert_eq!(old_value, None);
}

#[test]
fn test_insert_full_occupied_entry() {
    struct TestEntries;
    impl Entries for TestEntries {
        type Entry = Bucket<u32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            vec![Bucket { hash: HashValue(42), key: 1, value: "original".to_string() }]
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }

        fn with_entries<F>(&mut self, _f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {}
    }

    let mut index_map = IndexMapCore::<u32, String>::new();
    index_map.insert_full(HashValue(42), 1, "original".to_string());

    let (index, old_value) = index_map.insert_full(HashValue(42), 1, "new_value".to_string());
    
    assert_eq!(index, 0);
    assert_eq!(old_value, Some("original".to_string()));
}

