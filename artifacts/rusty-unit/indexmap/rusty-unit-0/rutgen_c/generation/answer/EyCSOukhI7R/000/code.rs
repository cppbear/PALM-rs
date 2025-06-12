// Answer 0

#[test]
fn test_as_mut_slice() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
    ];

    let mut iter = IntoIter::new(buckets);
    let slice = iter.as_mut_slice();

    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key.0, 1);
    assert_eq!(slice.entries[0].value.0, 10);
    assert_eq!(slice.entries[1].key.0, 2);
    assert_eq!(slice.entries[1].value.0, 20);
}

#[test]
fn test_as_mut_slice_empty() {
    struct TestKey(i32);
    struct TestValue(i32);

    let buckets: Vec<Bucket<TestKey, TestValue>> = Vec::new();
    let mut iter = IntoIter::new(buckets);
    
    let slice = iter.as_mut_slice();
    assert_eq!(slice.entries.len(), 0);
}

