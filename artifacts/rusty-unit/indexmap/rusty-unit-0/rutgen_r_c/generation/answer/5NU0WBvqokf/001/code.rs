// Answer 0

#[test]
fn test_push_entry_with_full_capacity() {
    struct TestEntries {
        entries: Vec<Bucket<usize, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<usize, String>;
        
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
                f(&mut self.entries);
        }
    }

    let mut map_core = IndexMapCore::<usize, String>::with_capacity(1);
    let hash_value = HashValue(1);
    let key = 42;
    let value = "test".to_string();

    // Fill the entries to reach capacity
    map_core.push_entry(hash_value, key, value.clone());
    
    // Assertions to ensure entries are at capacity and that pushing an additional entry will work
    assert_eq!(map_core.entries.len(), 1);
    assert_eq!(map_core.entries.capacity(), 1);

    // Attempting to push another entry should succeed but we need to check the value correctly pushed
    map_core.push_entry(HashValue(2), 43, "another test".to_string());

    assert_eq!(map_core.entries.len(), 2);
    assert_eq!(map_core.entries[0].value, value);
    assert_eq!(map_core.entries[1].value, "another test");
}

