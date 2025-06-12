// Answer 0

#[test]
fn test_value_ref() {
    struct TestKey;
    struct TestValue {
        value: i32,
    }

    let bucket = Bucket {
        hash: HashValue(42),
        key: TestKey,
        value: TestValue { value: 100 },
    };

    let value_ref = bucket.value_ref();
    assert_eq!(value_ref.value, 100);
}

#[test]
fn test_value_ref_empty_value() {
    struct TestEmptyValue;

    let bucket = Bucket {
        hash: HashValue(1),
        key: TestKey,
        value: TestEmptyValue,
    };

    let value_ref = bucket.value_ref();
    // Since TestEmptyValue doesn't have any fields, we just check existence.
    assert!(!std::ptr::is_null(value_ref));
}

