// Answer 0

#[test]
fn test_iter_mut_empty() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [],
    };

    let mut iter = slice.iter_mut();
    assert_eq!(iter.as_slice().len(), 0);
}

#[test]
fn test_iter_mut_single_entry() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [Bucket {
            hash: Default::default(),
            key: TestKey,
            value: TestValue,
        }],
    };

    let mut iter = slice.iter_mut();
    let bucket = iter.as_slice().get_index_mut(0).unwrap();

    assert!(bucket.is_some());
    assert_eq!(iter.as_slice().len(), 1);
}

#[test]
fn test_iter_mut_multiple_entries() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [
            Bucket {
                hash: Default::default(),
                key: TestKey,
                value: TestValue,
            },
            Bucket {
                hash: Default::default(),
                key: TestKey,
                value: TestValue,
            },
        ],
    };

    let mut iter = slice.iter_mut();
    assert_eq!(iter.as_slice().len(), 2);
    
    for _ in 0..2 {
        assert!(iter.as_slice().get_index_mut(0).is_some());
    }
}

