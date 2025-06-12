// Answer 0

#[test]
fn test_slice_index() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 123, key: 0, value: 100 },
        TestBucket { hash: 456, key: 1, value: 200 },
        TestBucket { hash: 789, key: 2, value: 300 },
    ];
    
    let slice = Slice { entries };

    assert_eq!(slice[index(0)], 0);
    assert_eq!(slice[index(1)], 1);
    assert_eq!(slice[index(2)], 2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_slice_index_out_of_bounds() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: usize,
    }

    let entries = [
        TestBucket { hash: 123, key: 0, value: 100 },
        TestBucket { hash: 456, key: 1, value: 200 },
    ];
    
    let slice = Slice { entries };

    let _ = slice[index(2)];
}

