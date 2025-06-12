// Answer 0

#[test]
fn test_get_disjoint_mut_valid_indices() {
    #[derive(Debug)]
    struct TestKey(usize);
    #[derive(Debug)]
    struct TestValue(usize);
    
    let mut slice = Slice::<TestKey, TestValue> {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(0), value: TestValue(100) },
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(200) },
            Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(300) },
        ],
    };
    
    let result = slice.get_disjoint_mut([0, 1]);
    assert!(result.is_ok());
    
    let result = result.unwrap();
    assert_eq!(result[0], (&TestKey(0), &mut slice.entries[0].value));
    assert_eq!(result[1], (&TestKey(1), &mut slice.entries[1].value));
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_invalid_index() {
    #[derive(Debug)]
    struct TestKey(usize);
    #[derive(Debug)]
    struct TestValue(usize);
    
    let mut slice = Slice::<TestKey, TestValue> {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(0), value: TestValue(100) },
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(200) },
            Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(300) },
        ],
    };

    // Trying to access index out of bounds (index 3)
    let _ = slice.get_disjoint_mut([0, 3]);
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_overlapping_indices() {
    #[derive(Debug)]
    struct TestKey(usize);
    #[derive(Debug)]
    struct TestValue(usize);
    
    let mut slice = Slice::<TestKey, TestValue> {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(0), value: TestValue(100) },
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(200) },
        ],
    };

    // Duplicate indices (0 and 0)
    let _ = slice.get_disjoint_mut([0, 0]);
}

