// Answer 0

#[test]
fn test_last_mut_non_empty() {
    struct TestKey(usize);
    struct TestValue(usize);

    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: TestKey(1), value: TestValue(10) },
            Bucket { hash: 1, key: TestKey(2), value: TestValue(20) },
        ],
    };

    let result = slice.last_mut();
    assert!(result.is_some());

    if let Some((key, value)) = result {
        assert_eq!(key.0, 2);
        value.0 += 5; // Modify value
        assert_eq!(value.0, 25);
    }
}

#[test]
fn test_last_mut_empty() {
    struct TestKey(usize);
    struct TestValue(usize);

    let mut slice = Slice { entries: [] };

    let result = slice.last_mut();
    assert!(result.is_none());
}

