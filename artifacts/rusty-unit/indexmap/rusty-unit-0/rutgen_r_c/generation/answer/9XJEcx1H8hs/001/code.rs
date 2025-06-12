// Answer 0

#[test]
fn test_split_first_mut_non_empty() {
    struct TestKey;
    struct TestValue(usize);

    let mut bucket1 = Bucket {
        hash: HashValue::default(),
        key: TestKey,
        value: TestValue(1),
    };
    let mut bucket2 = Bucket {
        hash: HashValue::default(),
        key: TestKey,
        value: TestValue(2),
    };

    let mut slice = Slice {
        entries: [bucket1, bucket2],
    };

    let result = slice.split_first_mut();
    assert!(result.is_some());

    if let Some((first_pair, rest_slice)) = result {
        assert_eq!(first_pair.0, &TestKey);
        assert_eq!(first_pair.1.0, 1);
        assert_eq!(rest_slice.len(), 1);
    }
}

#[test]
fn test_split_first_mut_empty() {
    struct TestKey;
    struct TestValue(usize);

    let slice: Slice<TestKey, TestValue> = Slice {
        entries: [],
    };

    let result = slice.split_first_mut();
    assert!(result.is_none());
}

