// Answer 0

#[test]
fn test_bucket_value() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: HashValue(1),
        key: TestKey,
        value: TestValue,
    };

    let value = bucket.value();
    assert_eq!(std::mem::discriminant(&value), std::mem::discriminant(&TestValue));
}

#[test]
fn test_bucket_value_boundary() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: HashValue(0),
        key: TestKey,
        value: TestValue,
    };

    let value = bucket.value();
    assert_eq!(std::mem::discriminant(&value), std::mem::discriminant(&TestValue));
}

