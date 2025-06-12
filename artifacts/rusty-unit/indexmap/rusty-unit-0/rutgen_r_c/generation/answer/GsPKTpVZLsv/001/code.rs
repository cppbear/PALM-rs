// Answer 0

#[test]
fn test_iter_mut() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: TestKey, value: TestValue },
            Bucket { hash: 1, key: TestKey, value: TestValue },
            Bucket { hash: 2, key: TestKey, value: TestValue },
        ],
    };

    let mut iter = slice.iter_mut();
    let entry1 = iter.iter.next().unwrap();
    assert_eq!(entry1.key, TestKey);
    assert_eq!(entry1.value, TestValue);
    
    let entry2 = iter.iter.next().unwrap();
    assert_eq!(entry2.key, TestKey);
    assert_eq!(entry2.value, TestValue);
    
    let entry3 = iter.iter.next().unwrap();
    assert_eq!(entry3.key, TestKey);
    assert_eq!(entry3.value, TestValue);
    
    assert!(iter.iter.next().is_none());
}

#[test]
fn test_iter_mut_empty() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [],
    };

    let mut iter = slice.iter_mut();
    assert!(iter.iter.next().is_none());
}

#[test]
fn test_iter_mut_single_entry() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [Bucket { hash: 0, key: TestKey, value: TestValue }],
    };

    let mut iter = slice.iter_mut();
    let entry1 = iter.iter.next().unwrap();
    assert_eq!(entry1.key, TestKey);
    assert_eq!(entry1.value, TestValue);
    
    assert!(iter.iter.next().is_none());
}

