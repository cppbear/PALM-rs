// Answer 0

#[test]
fn test_index_with_valid_index() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: 0,
        key: TestKey,
        value: TestValue,
    };
    
    let buckets = vec![bucket];
    let keys = Keys {
        iter: buckets.iter(),
    };

    let result = keys.index(0);
    assert_eq!(std::ptr::eq(result, &bucket.key), true);
}

#[should_panic]
fn test_index_with_out_of_bounds_index() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: 0,
        key: TestKey,
        value: TestValue,
    };

    let buckets = vec![bucket];
    let keys = Keys {
        iter: buckets.iter(),
    };

    let _ = keys.index(1); // Out of bounds
}

