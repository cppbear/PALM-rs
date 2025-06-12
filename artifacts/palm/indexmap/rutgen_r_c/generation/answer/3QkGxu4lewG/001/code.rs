// Answer 0

#[test]
fn test_from_mut_slice() {
    struct TestKey;
    struct TestValue;

    let mut buckets: [Bucket<TestKey, TestValue>; 3] = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];

    let slice: &mut Slice<TestKey, TestValue> = Slice::from_mut_slice(&mut buckets);
    assert_eq!(slice.entries.len(), 3);
}

#[test]
#[should_panic]
fn test_from_mut_slice_empty() {
    struct TestKey;
    struct TestValue;

    let mut buckets: [Bucket<TestKey, TestValue>; 0] = [];
    let _slice: &mut Slice<TestKey, TestValue> = Slice::from_mut_slice(&mut buckets);
}

#[test]
fn test_from_mut_slice_single_element() {
    struct TestKey;
    struct TestValue;

    let mut buckets: [Bucket<TestKey, TestValue>; 1] = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
    ];

    let slice: &mut Slice<TestKey, TestValue> = Slice::from_mut_slice(&mut buckets);
    assert_eq!(slice.entries.len(), 1);
}

