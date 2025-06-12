// Answer 0

#[test]
fn test_index_with_valid_index() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }
    
    let entries = [
        TestBucket { hash: 1, key: 1, value: "first".to_string() },
        TestBucket { hash: 2, key: 2, value: "second".to_string() },
        TestBucket { hash: 3, key: 3, value: "third".to_string() },
    ];
    
    let slice = Slice { entries };
    
    assert_eq!(slice.index(0), "first");
    assert_eq!(slice.index(1), "second");
    assert_eq!(slice.index(2), "third");
}

#[test]
#[should_panic(expected = "index out of bounds: the length is 3 but the index is 3")]
fn test_index_with_out_of_bounds_index() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }
    
    let entries = [
        TestBucket { hash: 1, key: 1, value: "first".to_string() },
        TestBucket { hash: 2, key: 2, value: "second".to_string() },
        TestBucket { hash: 3, key: 3, value: "third".to_string() },
    ];
    
    let slice = Slice { entries };
    
    let _ = slice.index(3); // out of bounds access
}

#[test]
#[should_panic(expected = "index out of bounds: the length is 3 but the index is 10")]
fn test_index_with_large_index() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }
    
    let entries = [
        TestBucket { hash: 1, key: 1, value: "first".to_string() },
        TestBucket { hash: 2, key: 2, value: "second".to_string() },
        TestBucket { hash: 3, key: 3, value: "third".to_string() },
    ];
    
    let slice = Slice { entries };
    
    let _ = slice.index(10); // out of bounds access
}

#[test]
fn test_index_with_zero_index() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }
    
    let entries = [
        TestBucket { hash: 1, key: 1, value: "zero".to_string() },
    ];
    
    let slice = Slice { entries };
    
    assert_eq!(slice.index(0), "zero");
}

