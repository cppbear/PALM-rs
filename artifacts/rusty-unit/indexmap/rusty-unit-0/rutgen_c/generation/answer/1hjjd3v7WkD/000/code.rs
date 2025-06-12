// Answer 0

#[test]
fn test_new_mut_empty_slice() {
    struct TestKey;
    struct TestValue;

    let slice: &mut Slice<TestKey, TestValue> = Slice::new_mut();
    assert!(slice.entries.is_empty());
}

#[test]
fn test_new_mut_non_empty_slice() {
    struct TestKey;
    struct TestValue;

    let mut entries: Vec<Bucket<TestKey, TestValue>> = Vec::new();
    entries.push(Bucket { hash: 0, key: TestKey, value: TestValue });
    let slice: &mut Slice<TestKey, TestValue> = Slice::from_mut_slice(&mut entries);
    assert!(!slice.entries.is_empty());
}

