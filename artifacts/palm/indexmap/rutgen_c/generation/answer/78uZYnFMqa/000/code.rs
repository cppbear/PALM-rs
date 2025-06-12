// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: i32,
    }

    let entries = [
        Bucket { hash: 1, key: 10, value: 100 },
        Bucket { hash: 2, key: 20, value: 200 },
        Bucket { hash: 3, key: 30, value: 300 },
    ];

    let slice = Slice { entries };

    assert_eq!(slice.get_index(0), Some(&10));
    assert_eq!(slice.get_index(1), Some(&20));
    assert_eq!(slice.get_index(2), Some(&30));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: i32,
    }

    let entries = [
        Bucket { hash: 1, key: 10, value: 100 },
        Bucket { hash: 2, key: 20, value: 200 },
    ];

    let slice = Slice { entries };

    assert_eq!(slice.get_index(2), None);
    assert_eq!(slice.get_index(3), None);
}

#[test]
fn test_get_index_empty() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: i32,
    }

    let entries: [Bucket<TestBucket>; 0] = [];
    let slice = Slice { entries };

    assert_eq!(slice.get_index(0), None);
}

