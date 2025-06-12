// Answer 0

#[test]
fn test_last_mut_with_non_empty_slice() {
    struct TestKey(usize);
    struct TestValue(String);
    
    let mut slice = Slice {
        entries: [
            Bucket { hash: 1, key: TestKey(1), value: TestValue("Value 1".to_string()) },
            Bucket { hash: 2, key: TestKey(2), value: TestValue("Value 2".to_string()) },
        ],
    };

    if let Some((key, value)) = slice.last_mut() {
        assert_eq!(key.0, 2);
        value.0.push_str(" updated");
        assert_eq!(value.0, "Value 2 updated");
    } else {
        panic!("Expected to get a key-value pair from last_mut");
    }
}

#[test]
fn test_last_mut_with_empty_slice() {
    struct TestKey(usize);
    struct TestValue(String);
    
    let mut slice = Slice { entries: [] };

    assert!(slice.last_mut().is_none());
}

