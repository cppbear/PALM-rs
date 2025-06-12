// Answer 0

#[test]
fn test_reverse_with_entries() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }
    
    impl TestEntries {
        fn new() -> Self {
            TestEntries { entries: Vec::new() }
        }

        fn reverse(&mut self) {
            self.entries.reverse();
        }
    }
    
    let mut index_map_core = IndexMapCore {
        indices: Indices::new(),
        entries: TestEntries::new(),
    };

    // Adding entries manually to test the reverse function.
    index_map_core.entries.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map_core.entries.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    index_map_core.entries.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });

    // Simulate indices for the entries for testing.
    index_map_core.indices.push(0);
    index_map_core.indices.push(1);
    index_map_core.indices.push(2);

    index_map_core.reverse();
    
    // Check if the entries have been reversed correctly.
    assert_eq!(index_map_core.entries.entries[0].key, 3);
    assert_eq!(index_map_core.entries.entries[1].key, 2);
    assert_eq!(index_map_core.entries.entries[2].key, 1);
}

#[test]
fn test_reverse_no_entries() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            TestEntries { entries: Vec::new() }
        }

        fn reverse(&mut self) {
            self.entries.reverse();
        }
    }

    let mut index_map_core = IndexMapCore {
        indices: Indices::new(),
        entries: TestEntries::new(),
    };

    // No entries added, testing reverse should not panic.
    index_map_core.reverse();
    assert_eq!(index_map_core.entries.entries.len(), 0);
} 

#[test]
#[should_panic]
fn test_reverse_with_invalid_indices() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            TestEntries { entries: Vec::new() }
        }

        fn reverse(&mut self) {
            self.entries.reverse();
        }
    }

    let mut index_map_core = IndexMapCore {
        indices: Indices::new(),
        entries: TestEntries::new(),
    };

    index_map_core.entries.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });

    // Add an invalid index which exceeds the number of entries.
    index_map_core.indices.push(10); // This is invalid.
    index_map_core.reverse(); // This should panic due to invalid index access.
}

