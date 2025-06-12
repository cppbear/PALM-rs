// Answer 0

#[test]
fn test_into_keys_new_with_entries() {
    #[derive(Copy, Debug, Hash, PartialEq, Eq)]
    struct TestKey(u32);
    
    #[derive(Debug, PartialEq)]
    struct TestValue(String);

    let buckets = vec![
        Bucket { hash: HashValue::new(1), key: TestKey(1), value: TestValue("value1".to_string()) },
        Bucket { hash: HashValue::new(2), key: TestKey(2), value: TestValue("value2".to_string()) },
    ];

    let into_keys = IntoKeys::new(buckets.clone());
    
    // Verify that the iterator length matches the original buckets
    assert_eq!(into_keys.iter.len(), buckets.len());
}

#[test]
fn test_into_keys_new_empty() {
    let buckets: Vec<Bucket<u32, String>> = vec![];
    
    let into_keys = IntoKeys::new(buckets);
    
    // Verify that the iterator is empty
    assert_eq!(into_keys.iter.len(), 0);
}

