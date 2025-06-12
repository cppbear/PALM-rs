// Answer 0

#[test]
fn test_from_slice_valid_entries() {
    struct TestKey(usize);
    struct TestValue(String);

    let buckets: &mut [Bucket<TestKey, TestValue>] = &mut [
        Bucket { hash: 0, key: TestKey(1), value: TestValue("value1".to_string()) },
        Bucket { hash: 0, key: TestKey(2), value: TestValue("value2".to_string()) },
    ];

    let slice = Slice::<TestKey, TestValue>::from_slice(buckets);
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key.0, 1);
    assert_eq!(slice.entries[1].key.0, 2);
}

#[test]
fn test_from_slice_empty_entries() {
    struct TestKey(usize);
    struct TestValue(String);

    let buckets: &mut [Bucket<TestKey, TestValue>] = &mut [];

    let slice = Slice::<TestKey, TestValue>::from_slice(buckets);
    assert_eq!(slice.entries.len(), 0);
}

#[should_panic]
fn test_from_slice_invalid_reference() {
    struct TestKey(usize);
    struct TestValue(String);

    {
        let buckets: &mut [Bucket<TestKey, TestValue>] = &mut [
            Bucket { hash: 0, key: TestKey(1), value: TestValue("value1".to_string()) },
        ];
        let _slice = Slice::<TestKey, TestValue>::from_slice(buckets);
    }
    
    // Attempt to access it after the scope is dropped
    let buckets: &mut [Bucket<TestKey, TestValue>] = &mut [Bucket { hash: 0, key: TestKey(1), value: TestValue("value1".to_string()) }];
    let _slice = Slice::<TestKey, TestValue>::from_slice(buckets);
    // The reference should not be valid here as `buckets` is out of scope
}

