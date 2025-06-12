// Answer 0

#[test]
fn test_erase_index_found() {
    struct TestEntry {
        index: usize,
        hash_value: u64,
    }

    impl HashValue {
        fn new(val: usize) -> Self {
            HashValue(val)
        }
    }

    let mut table = hashbrown::HashMap::new();
    let hash = HashValue::new(1);
    let index = 0;

    // Initialize Entries
    table.insert(hash.get(), index);

    // Call the function - should not panic
    erase_index(&mut table, hash, index);

    // Verify the index has been erased
    assert!(table.get(&hash.get()).is_none());
}

#[should_panic(expected = "index not found")]
#[test]
fn test_erase_index_not_found() {
    struct TestEntry {
        index: usize,
        hash_value: u64,
    }

    impl HashValue {
        fn new(val: usize) -> Self {
            HashValue(val)
        }
    }

    let mut table = hashbrown::HashMap::new();
    let hash = HashValue::new(2);
    let index = 1;

    // Initialize Entries
    table.insert(hash.get(), 0); // different index

    // Call the function - should panic because index not found
    erase_index(&mut table, hash, index);
}

#[test]
fn test_erase_index_empty_table() {
    struct TestEntry {
        index: usize,
        hash_value: u64,
    }

    impl HashValue {
        fn new(val: usize) -> Self {
            HashValue(val)
        }
    }

    let mut table = hashbrown::HashMap::new();
    let hash = HashValue::new(3);
    let index = 0;

    // Call the function - should panic because the table is empty
    let result = std::panic::catch_unwind(|| {
        erase_index(&mut table, hash, index);
    });
    
    assert!(result.is_err());
}

