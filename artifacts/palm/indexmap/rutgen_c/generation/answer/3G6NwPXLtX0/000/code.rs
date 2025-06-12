// Answer 0

#[test]
fn test_muts() {
    struct TestKey;
    struct TestValue;

    let mut bucket = Bucket {
        hash: HashValue(0),
        key: TestKey,
        value: TestValue,
    };

    let (key_mut, value_mut) = bucket.muts();
    
    // Validate that we can mutate the value
    *value_mut = TestValue; // assuming implementation allows this type of assignment
    assert!(std::ptr::eq(key_mut, &bucket.key));
    assert!(std::ptr::eq(value_mut, &bucket.value));
}

#[test]
fn test_muts_multiple_calls() {
    struct KeyType;
    struct ValueType;

    let mut bucket = Bucket {
        hash: HashValue(1),
        key: KeyType,
        value: ValueType,
    };

    let (key1, value1) = bucket.muts();
    let (key2, value2) = bucket.muts();

    assert!(std::ptr::eq(key1, key2));
    assert!(std::ptr::eq(value1, value2));
}

