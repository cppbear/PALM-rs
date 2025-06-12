// Answer 0

#[test]
fn test_iter_new_with_non_empty_bucket_array() {
    struct TestKey;
    struct TestValue;

    let buckets = [
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];
    
    let iter = Iter::new(&buckets);
    let slice = iter.iter.as_slice();
    assert_eq!(slice.len(), 2);
}

#[test]
fn test_iter_new_with_empty_bucket_array() {
    struct TestKey;
    struct TestValue;

    let buckets: Vec<Bucket<TestKey, TestValue>> = Vec::new();
    
    let iter = Iter::new(&buckets);
    let slice = iter.iter.as_slice();
    assert_eq!(slice.len(), 0);
}

