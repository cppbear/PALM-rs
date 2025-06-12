// Answer 0

#[test]
fn test_last_non_empty_slice() {
    struct TestKey;
    struct TestValue;

    let bucket1 = Bucket { hash: 0, key: TestKey, value: TestValue };
    let bucket2 = Bucket { hash: 1, key: TestKey, value: TestValue };
    
    let slice = Slice { entries: [bucket1, bucket2] };
    let last_entry = slice.last();

    assert!(last_entry.is_some());
}

#[test]
fn test_last_empty_slice() {
    struct TestKey;
    struct TestValue;

    let slice: Slice<TestKey, TestValue> = Slice { entries: [] };
    let last_entry = slice.last();

    assert!(last_entry.is_none());
}

