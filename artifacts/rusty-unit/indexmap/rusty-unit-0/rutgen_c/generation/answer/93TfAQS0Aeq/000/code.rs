// Answer 0

#[test]
fn test_split_last_mut_non_empty() {
    // Define a simple structure for testing
    struct TestKey(usize);
    struct TestValue(String);

    // Create a Slice with a few entries
    let mut entries = vec![
        Bucket { hash: 0, key: TestKey(1), value: TestValue("value1".to_string()) },
        Bucket { hash: 0, key: TestKey(2), value: TestValue("value2".to_string()) },
    ];
    let mut slice = Slice { entries: entries };

    // Call the function under test
    let result = slice.split_last_mut();

    // Verify the result
    assert!(result.is_some());
    let (last_pair, rest) = result.unwrap();
    assert_eq!(last_pair.0 .0, 2);
    assert_eq!(last_pair.1 .0, "value2");
    assert_eq!(rest.len(), 1);
}

#[test]
fn test_split_last_mut_empty() {
    // Define a simple structure for testing
    struct TestKey(usize);
    struct TestValue(String);

    // Create an empty Slice
    let mut entries: Vec<Bucket<TestKey, TestValue>> = Vec::new();
    let mut slice = Slice { entries };

    // Call the function under test
    let result = slice.split_last_mut();

    // Verify the result
    assert!(result.is_none());
}

