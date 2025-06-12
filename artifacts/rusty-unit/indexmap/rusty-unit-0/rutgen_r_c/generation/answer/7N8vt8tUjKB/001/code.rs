// Answer 0

#[test]
fn test_key_value() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: HashValue(1),
        key: TestKey,
        value: TestValue,
    };

    let (key, value) = bucket.key_value();
    assert_eq!(key, bucket.key);
    assert_eq!(value, bucket.value);
}

