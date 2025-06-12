// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestKey;
    struct TestValue;

    let buckets = [
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];

    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();

    assert_eq!(slice.entries.len(), buckets.len());
}

#[test]
fn test_as_slice_empty() {
    struct TestKey;
    struct TestValue;

    let buckets: Vec<Bucket<TestKey, TestValue>> = Vec::new();

    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();

    assert_eq!(slice.entries.len(), 0);
}

#[should_panic]
fn test_as_slice_invalid_panic() {
    struct TestKey;
    struct TestValue;

    let mut buckets = vec![
        Bucket { hash: 1, key: TestKey, value: TestValue },
    ];

    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();

    // This should panic if the underlying slice is modified improperly
    buckets.clear();
    let _ = iter.as_slice();
}

#[test]
fn test_as_slice_single_entry() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket { hash: 1, key: TestKey, value: TestValue };
    let buckets = [bucket];

    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();

    assert_eq!(slice.entries.len(), 1);
    assert_eq!(slice.entries[0].hash, bucket.hash);
}

