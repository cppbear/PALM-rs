// Answer 0

#[test]
fn test_values_mut_non_empty() {
    struct TestKey;
    struct TestValue(u32);

    let mut entries: [Bucket<TestKey, TestValue>; 2] = [
        Bucket { hash: 1, key: TestKey, value: TestValue(1) },
        Bucket { hash: 2, key: TestKey, value: TestValue(2) },
    ];

    let mut slice = Slice { entries };

    let mut values_mut = slice.values_mut();
    if let Some((_, value)) = values_mut.iter.next() {
        value.0 = 10;
    }

    assert_eq!(slice.entries[0].value.0, 10);
}

#[test]
fn test_values_mut_empty() {
    struct TestKey;
    struct TestValue(u32);

    let mut entries: [Bucket<TestKey, TestValue>; 0] = [];
    let mut slice = Slice { entries };

    let mut values_mut = slice.values_mut();
    assert!(values_mut.iter.next().is_none());
}

