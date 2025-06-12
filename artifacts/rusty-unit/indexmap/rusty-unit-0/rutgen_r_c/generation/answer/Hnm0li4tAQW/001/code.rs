// Answer 0

#[test]
fn test_iter_non_empty() {
    struct TestKey;
    struct TestValue;
    
    let slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue(1), key: TestKey, value: TestValue },
            Bucket { hash: HashValue(2), key: TestKey, value: TestValue },
        ],
    });

    let iter = slice.iter();
    
    assert_eq!(iter.as_slice().entries.len(), 2);
}

#[test]
fn test_iter_empty() {
    struct TestKey;
    struct TestValue;

    let slice = Box::new(Slice {
        entries: [],
    });

    let iter = slice.iter();

    assert_eq!(iter.as_slice().entries.len(), 0);
}

#[test]
fn test_iter_single_element() {
    struct TestKey;
    struct TestValue;

    let slice = Box::new(Slice {
        entries: [Bucket { hash: HashValue(1), key: TestKey, value: TestValue }],
    });

    let iter = slice.iter();

    assert_eq!(iter.as_slice().entries.len(), 1);
}

