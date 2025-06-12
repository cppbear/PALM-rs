// Answer 0

#[test]
fn test_bucket_refs() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: HashValue(0),
        key: TestKey,
        value: TestValue,
    };

    let (key_ref, value_ref) = bucket.refs();
    
    assert_eq!(key_ref, &bucket.key);
    assert_eq!(value_ref, &bucket.value);
}

