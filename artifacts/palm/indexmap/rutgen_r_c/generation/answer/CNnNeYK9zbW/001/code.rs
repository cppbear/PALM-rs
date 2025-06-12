// Answer 0

#[test]
fn test_get_full_mut_key_not_present() {
    struct TestEntry {
        key: String,
        value: i32,
    }

    struct TestIndexMap {
        entries: Vec<TestEntry>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            None // Simulates that the key is not found
        }

        fn as_entries_mut(&mut self) -> &mut [TestEntry] {
            &mut self.entries
        }
        
        pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &String, &mut i32)>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some((i, &entry.key, &mut entry.value))
            } else {
                None
            }
        }
    }

    let mut map = TestIndexMap::new();
    let key_to_test = "non_existent_key".to_string();

    let result = map.get_full_mut(&key_to_test);
    assert!(result.is_none());
}

