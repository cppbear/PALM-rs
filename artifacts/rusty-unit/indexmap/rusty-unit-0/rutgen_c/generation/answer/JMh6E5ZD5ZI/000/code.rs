// Answer 0

#[test]
fn test_key_ref() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: HashValue(123),
        key: TestKey,
        value: TestValue,
    };

    let key_ref = bucket.key_ref();
    assert_eq!(key_ref, &bucket.key);
}

