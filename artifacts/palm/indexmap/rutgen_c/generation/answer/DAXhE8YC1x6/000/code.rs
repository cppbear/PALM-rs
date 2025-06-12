// Answer 0

#[test]
fn test_iter_empty() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }
    
    let slice: Box<Slice<TestBucket>> = Box::new(Slice { entries: [] });
    let iter = slice.iter();
    assert_eq!(iter.as_slice().entries.len(), 0);
}

#[test]
fn test_iter_non_empty() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        Bucket {
            hash: 1,
            key: 1,
            value: 10,
        },
        Bucket {
            hash: 2,
            key: 2,
            value: 20,
        },
    ];

    let slice: Box<Slice<TestBucket>> = Box::new(Slice { entries });
    let iter = slice.iter();
    assert_eq!(iter.as_slice().entries.len(), 2);
    assert_eq!(iter.as_slice().entries[0].key, 1);
    assert_eq!(iter.as_slice().entries[1].key, 2);
}

