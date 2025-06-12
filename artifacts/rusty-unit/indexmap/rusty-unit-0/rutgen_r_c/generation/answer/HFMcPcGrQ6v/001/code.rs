// Answer 0

#[test]
fn test_is_empty_with_non_empty_slice() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 1, key: 1, value: 10 },
        TestBucket { hash: 2, key: 2, value: 20 },
    ];
    let slice = Slice { entries };

    assert!(!slice.is_empty());
}

#[test]
fn test_is_empty_with_empty_slice() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries: [TestBucket; 0] = [];
    let slice = Slice { entries };

    assert!(slice.is_empty());
}

