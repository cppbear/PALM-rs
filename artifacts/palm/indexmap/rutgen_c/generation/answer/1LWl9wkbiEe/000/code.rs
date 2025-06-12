// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries = [
        Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
    ];
    
    let slice = Slice { entries };

    assert_eq!(slice.get_index(0), Some((&TestKey(1), &TestValue(10))));
    assert_eq!(slice.get_index(1), Some((&TestKey(2), &TestValue(20))));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries = [
        Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
    ];
    
    let slice = Slice { entries };

    assert_eq!(slice.get_index(1), None);
}

#[test]
fn test_get_index_empty() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries: [Bucket<TestKey, TestValue>; 0] = [];
    
    let slice = Slice { entries };

    assert_eq!(slice.get_index(0), None);
}

