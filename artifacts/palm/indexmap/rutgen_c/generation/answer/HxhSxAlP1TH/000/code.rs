// Answer 0

#[test]
fn test_split_at_with_valid_index() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let bucket1 = Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) };
    let bucket2 = Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) };
    let slice = Slice { entries: [bucket1, bucket2] };

    let (first, second) = slice.split_at(1);
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 1);
    assert_eq!(first.get_index(0).unwrap().0, &TestKey(1));
    assert_eq!(second.get_index(0).unwrap().0, &TestKey(2));
}

#[test]
#[should_panic]
fn test_split_at_with_index_greater_than_length() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let bucket1 = Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) };
    let slice = Slice { entries: [bucket1] };

    slice.split_at(2); // This should panic
}

#[test]
fn test_split_at_with_index_equal_to_length() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let bucket1 = Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) };
    let bucket2 = Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) };
    let slice = Slice { entries: [bucket1, bucket2] };

    let (first, second) = slice.split_at(2);
    assert!(first.len() == 2);
    assert!(second.is_empty());
}

