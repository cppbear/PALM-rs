// Answer 0

#[test]
fn test_split_last_non_empty() {
    struct TestKey(i32);
    let bucket1 = Bucket { hash: 0, key: TestKey(1), value: "a" };
    let bucket2 = Bucket { hash: 0, key: TestKey(2), value: "b" };
    let bucket3 = Bucket { hash: 0, key: TestKey(3), value: "c" };
    let slice = Slice {
        entries: [bucket1, bucket2, bucket3],
    };
    
    let result = slice.split_last();
    assert!(result.is_some());

    let (last_key, rest_slice) = result.unwrap();
    assert_eq!(last_key, &TestKey(3));
    assert_eq!(rest_slice.len(), 2);
}

#[test]
fn test_split_last_empty() {
    let slice: &Slice<i32> = Slice::new();
    
    let result = slice.split_last();
    assert!(result.is_none());
}

