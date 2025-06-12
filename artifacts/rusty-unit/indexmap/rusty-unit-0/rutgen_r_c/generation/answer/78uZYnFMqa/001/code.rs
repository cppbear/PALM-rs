// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestBucket {
        key: usize,
    }

    impl Bucket<TestBucket, usize> {
        fn new(key: usize, value: usize) -> Self {
            Bucket { hash: 0, key, value }
        }
    }

    let entries = vec![
        TestBucket::new(0, 10),
        TestBucket::new(1, 20),
        TestBucket::new(2, 30),
    ];

    let slice = Slice { entries };

    assert_eq!(slice.get_index(0).map(|b| b.key), Some(0));
    assert_eq!(slice.get_index(1).map(|b| b.key), Some(1));
    assert_eq!(slice.get_index(2).map(|b| b.key), Some(2));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct TestBucket {
        key: usize,
    }

    impl Bucket<TestBucket, usize> {
        fn new(key: usize, value: usize) -> Self {
            Bucket { hash: 0, key, value }
        }
    }

    let entries = vec![
        TestBucket::new(0, 10),
        TestBucket::new(1, 20),
        TestBucket::new(2, 30),
    ];

    let slice = Slice { entries };

    assert_eq!(slice.get_index(3), None);
    assert_eq!(slice.get_index(usize::MAX), None);
}

#[test]
fn test_get_index_empty_slice() {
    struct TestBucket {
        key: usize,
    }

    impl Bucket<TestBucket, usize> {
        fn new(key: usize, value: usize) -> Self {
            Bucket { hash: 0, key, value }
        }
    }

    let entries: Vec<TestBucket> = vec![];
    let slice = Slice { entries };

    assert_eq!(slice.get_index(0), None);
}

