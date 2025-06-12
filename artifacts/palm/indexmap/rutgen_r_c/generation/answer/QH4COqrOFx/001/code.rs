// Answer 0

#[test]
fn test_last_non_empty_slice() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 1, key: 1, value: 10 },
        TestBucket { hash: 2, key: 2, value: 20 },
        TestBucket { hash: 3, key: 3, value: 30 },
    ];

    let slice = Slice { entries };

    assert_eq!(slice.last(), Some(&TestBucket { hash: 3, key: 3, value: 30 }));
}

#[test]
fn test_last_empty_slice() {
    let entries: [Bucket<usize>; 0] = [];
    let slice = Slice { entries };

    assert_eq!(slice.last(), None);
}

#[test]
fn test_last_single_element_slice() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 1, key: 1, value: 10 },
    ];
    
    let slice = Slice { entries };

    assert_eq!(slice.last(), Some(&TestBucket { hash: 1, key: 1, value: 10 }));
}

