// Answer 0

#[test]
fn test_iter_empty_slice() {
    struct TestKey;
    struct TestValue;

    let slice = Slice::<TestKey, TestValue>::new();
    let mut iter = slice.iter();
    
    assert_eq!(iter.as_slice().entries.len(), 0);
}

#[test]
fn test_iter_non_empty_slice() {
    struct TestKey;
    struct TestValue;

    let entry1 = Bucket { hash: 0, key: TestKey, value: TestValue };
    let entry2 = Bucket { hash: 1, key: TestKey, value: TestValue };
    
    let entries = Box::new(Slice { entries: [entry1, entry2] });
    let slice = &*entries;

    let mut iter = slice.iter();
    
    assert_eq!(iter.as_slice().entries.len(), 2);
}

