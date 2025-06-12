// Answer 0

#[test]
fn test_get_range_valid() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 1, key: 0, value: 10 },
        TestBucket { hash: 2, key: 1, value: 20 },
        TestBucket { hash: 3, key: 2, value: 30 },
    ];
    let slice = Slice { entries };

    let result = slice.get_range(0..2);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 2);
}

#[test]
fn test_get_range_empty_slice() {
    let slice = Slice::new();

    let result = slice.get_range(0..1);
    assert!(result.is_none());
}

#[test]
fn test_get_range_out_of_bounds() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 1, key: 0, value: 10 },
        TestBucket { hash: 2, key: 1, value: 20 },
    ];
    let slice = Slice { entries };

    let result = slice.get_range(1..3);
    assert!(result.is_none());
}

#[test]
fn test_get_range_full_range() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 1, key: 0, value: 10 },
        TestBucket { hash: 2, key: 1, value: 20 },
    ];
    let slice = Slice { entries };

    let result = slice.get_range(..);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 2);
}

#[test]
fn test_get_range_inclusive_bounds() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 1, key: 0, value: 10 },
        TestBucket { hash: 2, key: 1, value: 20 },
        TestBucket { hash: 3, key: 2, value: 30 },
    ];
    let slice = Slice { entries };

    let result = slice.get_range(1..=2);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 2);
}

