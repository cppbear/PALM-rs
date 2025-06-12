// Answer 0

#[test]
fn test_retain_in_order() {
    struct TestEntry {
        key: usize,
        value: usize,
    }

    impl TestEntry {
        fn new(key: usize, value: usize) -> Self {
            Self { key, value }
        }
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    // Inserting entries to ensure we have some data.
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });

    // Simulate indices having more elements than entries
    map.indices.insert(0, 0);
    map.indices.insert(1, 1);
    map.indices.insert(2, 2);
    // Setting extra index to satisfy condition
    map.indices.insert(3, 3);

    // Retain function that keeps only even keys
    map.retain_in_order(|k, v| *k % 2 == 0);

    // After retention, there should be only one entry with key 2
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, 2);
    assert_eq!(map.entries[0].value, 20);
    assert_eq!(map.indices.len(), 4); // Ensures indices remain intact
}

#[test]
fn test_retain_in_order_with_no_elements() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    // Simulate indices having more elements than entries
    map.indices.insert(0, 0);
    map.indices.insert(1, 1);
    map.indices.insert(2, 2);
    map.indices.insert(3, 3);

    // Retain function that always returns false
    map.retain_in_order(|_, _| false);

    // After retention, entries should be empty
    assert_eq!(map.entries.len(), 0);
    assert_eq!(map.indices.len(), 4); // Indices remain intact
}

#[test]
#[should_panic]
fn test_retain_in_order_panic_condition() {
    struct TestEntry {
        key: usize,
        value: usize,
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    // Inserting entries to ensure we have some data and indices
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.indices.insert(0, 0);

    // This should trigger a panic due to the invariants of the function.
    map.retain_in_order(|_, _| false);
}

