// Answer 0

#[test]
fn test_first_non_empty_slice() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: i32,
    }

    let buckets = [
        TestBucket { hash: 1, key: 10, value: 100 },
        TestBucket { hash: 2, key: 20, value: 200 },
    ];
    
    let slice = Slice { entries: buckets };

    assert_eq!(slice.first(), Some(&10));
}

#[test]
fn test_first_empty_slice() {
    let buckets: [Bucket<i32, i32>; 0] = [];
    let slice = Slice { entries: buckets };

    assert_eq!(slice.first(), None);
}

