// Answer 0

#[test]
fn test_get_index_mut_valid_index() {
    struct TestKey;
    struct TestValue(i32);
    
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0.into(), key: TestKey, value: TestValue(1) },
            Bucket { hash: 0.into(), key: TestKey, value: TestValue(2) },
        ],
    };

    if let Some((_, value_mut)) = slice.get_index_mut(0) {
        value_mut.0 = 3;
    }
    assert_eq!(slice.entries[0].value.0, 3);
}

#[test]
fn test_get_index_mut_out_of_bounds() {
    struct TestKey;
    struct TestValue(i32);
    
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0.into(), key: TestKey, value: TestValue(1) },
        ],
    };

    let result = slice.get_index_mut(1);
    assert!(result.is_none());
}

