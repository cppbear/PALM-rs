// Answer 0

#[test]
fn test_with_entries() {
    struct TestEntry {
        key: String,
        value: i32,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [TestEntry]),
        {
            f(&mut self.entries);
            self.rebuild_hash_table();
        }

        fn rebuild_hash_table(&self) {
            // Simulating hash table rebuild
        }
    }

    let mut test_map = TestMap::new();
    test_map.with_entries(|entries| {
        entries.push(TestEntry {
            key: "test".to_string(),
            value: 42,
        });
    });

    assert_eq!(test_map.entries.len(), 1);
    assert_eq!(test_map.entries[0].key, "test");
    assert_eq!(test_map.entries[0].value, 42);
}

#[test]
fn test_with_entries_empty() {
    struct TestEntry {
        key: String,
        value: i32,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [TestEntry]),
        {
            f(&mut self.entries);
            self.rebuild_hash_table();
        }

        fn rebuild_hash_table(&self) {
            // Simulating hash table rebuild
        }
    }

    let mut test_map = TestMap::new();
    test_map.with_entries(|entries| {
        // No entries added
    });

    assert_eq!(test_map.entries.len(), 0);
}

