// Answer 0

#[test]
fn test_pop_with_entries() {
    #[derive(Debug)]
    struct TestKey(usize);
    #[derive(Debug)]
    struct TestValue(usize);

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    // Prepare the test data
    index_map.entries.push(Bucket {
        hash: HashValue(1),
        key: TestKey(1),
        value: TestValue(10),
    });

    index_map.entries.push(Bucket {
        hash: HashValue(2),
        key: TestKey(2),
        value: TestValue(20),
    });

    // Pop the last entry
    let result = index_map.pop();
    
    // Assert that the result matches the expected key-value pair
    assert_eq!(result, Some((TestKey(2), TestValue(20))));

    // Assert that one entry is removed from the index_map
    assert_eq!(index_map.entries.len(), 1);
}

#[test]
fn test_pop_empty() {
    #[derive(Debug)]
    struct TestKey(usize);
    #[derive(Debug)]
    struct TestValue(usize);

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    // Pop from an empty index_map
    let result = index_map.pop();
    
    // Assert that the result is None
    assert_eq!(result, None);
}

