// Answer 0

#[test]
fn test_keys_index_within_bounds() {
    struct TestKey;
    struct TestValue;

    let bucket1 = Bucket { hash: 1, key: TestKey, value: TestValue };
    let bucket2 = Bucket { hash: 2, key: TestKey, value: TestValue };
    let buckets = vec![bucket1, bucket2];
    let keys = Keys { iter: buckets.iter() };

    assert_eq!(std::ptr::eq(keys[0], &bucket1.key), true);
    assert_eq!(std::ptr::eq(keys[1], &bucket2.key), true);
}

#[test]
#[should_panic]
fn test_keys_index_out_of_bounds() {
    struct TestKey;
    struct TestValue;

    let bucket1 = Bucket { hash: 1, key: TestKey, value: TestValue };
    let buckets = vec![bucket1];
    let keys = Keys { iter: buckets.iter() };

    let _ = keys[1]; // This should panic as it is out of bounds
}

