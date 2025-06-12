// Answer 0

#[test]
fn test_split_first_non_empty() {
    struct TestKey;
    struct TestValue;

    let bucket1 = Bucket { hash: 0, key: TestKey, value: TestValue };
    let bucket2 = Bucket { hash: 1, key: TestKey, value: TestValue };
    let slice = Slice { entries: [bucket1, bucket2] };

    let result = slice.split_first();
    assert!(result.is_some());

    if let Some(((key, value), rest)) = result {
        assert_eq!(std::ptr::addr_of!(*key), std::ptr::addr_of!(&bucket1.key));
        assert_eq!(std::ptr::addr_of!(*value), std::ptr::addr_of!(&bucket1.value));
        assert_eq!(rest.len(), 1);
    }
}

#[test]
fn test_split_first_empty() {
    struct TestKey;
    struct TestValue;

    let slice: Slice<TestKey, TestValue> = Slice { entries: [] };
    let result = slice.split_first();
    assert!(result.is_none());
}

