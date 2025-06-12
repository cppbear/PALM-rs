// Answer 0

#[test]
fn test_first_non_empty_slice() {
    struct TestKey;

    struct TestValue;

    let bucket1 = Bucket { hash: 1, key: TestKey, value: TestValue };
    let bucket2 = Bucket { hash: 2, key: TestKey, value: TestValue };
    let slice = Slice { entries: [bucket1, bucket2] };

    assert!(slice.first().is_some());
}

#[test]
fn test_first_empty_slice() {
    let slice: Slice<i32, i32> = Slice { entries: [] };
    
    assert!(slice.first().is_none());
}

