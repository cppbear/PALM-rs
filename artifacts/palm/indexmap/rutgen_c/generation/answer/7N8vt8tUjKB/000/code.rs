// Answer 0

#[test]
fn test_key_value() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: HashValue(42),
        key: TestKey,
        value: TestValue,
    };

    let (key, value) = bucket.key_value();
    assert_eq!(std::mem::size_of::<TestKey>(), std::mem::size_of_val(&key));
    assert_eq!(std::mem::size_of::<TestValue>(), std::mem::size_of_val(&value));
}

#[test]
fn test_key_value_with_different_types() {
    struct AnotherKey(u32);
    struct AnotherValue(String);

    let bucket = Bucket {
        hash: HashValue(100),
        key: AnotherKey(1),
        value: AnotherValue("Test".to_string()),
    };

    let (key, value) = bucket.key_value();
    assert_eq!(key.0, 1);
    assert_eq!(value.0, "Test");
}

